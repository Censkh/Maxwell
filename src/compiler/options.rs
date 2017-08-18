extern crate json;

use std::io::prelude::*;
use std::fs::File;

use std::path::PathBuf;

use std::env;
use std::ops::Index;

//TODO: Cleanup move env vars away from this struct.

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(String),
    FileContentsInvalid,
    InvalidConfigOption(String, String),
    JsonInvalid,
}

#[derive(Debug, Clone)]
pub struct CompilerOptions {
    pub dir_path: PathBuf,
    config_name: String,
    config_path: PathBuf,

    pub src_base_path: PathBuf,
    pub entries: Vec<String>,

    pub out_dir: PathBuf,
    plugins: Vec<String>,
}

impl CompilerOptions {
    pub fn load() -> Result<Self, ConfigError> {
        let mut dir_path = env::current_dir().unwrap().to_path_buf();
        if env::args().len() >= 2 {
            dir_path.push(PathBuf::from(env::args().nth(1).unwrap()));
        }

        match dir_path.canonicalize() {
            Ok(path) => dir_path = path,
            Err(_) => {
                return Err(ConfigError::FileNotFound(dir_path.to_str().unwrap().to_owned()));
            }
        }

        let mut options = CompilerOptions {
            dir_path,
            config_name: "maxwell.json".to_owned(),
            config_path: PathBuf::default(),

            out_dir: PathBuf::default(),
            entries: Vec::new(),
            src_base_path: PathBuf::default(),
            plugins: Vec::new(),
        };

        if !options.dir_path.is_dir() {
            options.config_name = options.dir_path.file_name().unwrap().to_str().unwrap().to_string();
            options.dir_path = options.dir_path.parent().unwrap().to_path_buf();
        }

        options.config_path = options.dir_path.join(PathBuf::from(&options.config_name)).canonicalize().unwrap();

        match options.read_json() {
            Ok(config) => {
                match options.parse_json(config) {
                    Ok(_) => {}
                    Err(err) => return Err(err),
                };
            }
            Err(err) => return Err(err)
        }
        return Ok(options);
    }

    fn parse_json(&mut self, config: json::JsonValue) -> Result<(), ConfigError> {
        let out_element = config.index("out");
        let mut out_dir = self.dir_path.clone();
        for item in out_element.as_str().unwrap().split("/") {
            out_dir.push(item);
        }

        if !out_dir.is_absolute() {
            return Err(ConfigError::InvalidConfigOption(String::from("out"), String::from("Out directory is not valid")));
        }

        self.out_dir = out_dir;

        let base_dir_element = config.index("src").index("base");
        let mut base_dir = self.dir_path.clone();
        base_dir.push(PathBuf::from(base_dir_element.as_str().unwrap()));
        self.src_base_path = base_dir;
        if !self.src_base_path.is_absolute() {
            return Err(ConfigError::InvalidConfigOption(String::from("src.base"), String::from("Cannot find base directory")));
        }

        let entry_element = config.index("src").index("entry");
        if entry_element.is_string() {
            self.entries.push(entry_element.as_str().unwrap().to_owned());
        }

        return Ok(());
    }

    fn read_json(&self) -> Result<json::JsonValue, ConfigError> {
        let config_path = &self.config_path;
        let config_file_result = File::open(config_path);
        if config_file_result.is_err() {
            return Err(ConfigError::FileNotFound(config_path.to_str().unwrap().to_owned()));
        }
        let mut contents = String::new();
        if config_file_result.unwrap().read_to_string(&mut contents).is_err() {
            return Err(ConfigError::FileContentsInvalid);
        }

        return match json::parse(&contents) {
            Ok(config) => Ok(config),
            Err(_) => Err(ConfigError::JsonInvalid)
        };
    }
}