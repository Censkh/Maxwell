extern crate rayon;

use self::rayon::prelude::*;

use super::{Chunk, ChunkId, Generator, ChunkLocation, CompilerError, CompilerOptions};
use super::module::Resolver;
use super::parser::{JsParser, ParserError, ParserErrorKind, ParserResult, Parser, ParserOptions};
use super::ast::SourceLocation;

use super::transform::PluginManager;

use std::io::prelude::*;
use std::io::{Write, BufWriter};
use std::fs::File;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct CompileResult {
    pub overall_duration: Duration,
    pub emit_duration: Duration,
    pub parse_duration: Duration,
    pub emit_count: i16,
}

pub struct EmitResult {
    pub count: i16,
    pub duration: Duration,
}

pub struct Compiler {
    pub resolver: Resolver,
    chunk_map: HashMap<ChunkId, Chunk>,
    options: Option<CompilerOptions>,
    parser: Box<Parser>
}

impl Compiler {
    pub fn new() -> Self {
        return Compiler {
            resolver: Resolver {},
            chunk_map: HashMap::new(),
            options: None,
            parser: Box::new(JsParser::new())
        };
    }


    pub fn push_chunk(&mut self, chunk: Chunk) {
        let id = chunk.get_id();
        self.chunk_map.insert(id, chunk);
    }

    pub fn compile(&mut self, plugin_manager: PluginManager) -> Result<CompileResult, CompilerError> {
        let start = Instant::now();

        let options = match CompilerOptions::load() {
            Ok(options) => options,
            Err(err) => return Err(CompilerError::InvalidConfig(err))
        };

        if options.entries.len() == 0 {
            return Err(CompilerError::NoEntries);
        }

        let mut parse_duration = Duration::default();

        for entry in &options.entries {
            let location = ChunkLocation::absolute(options.dir_path.join(PathBuf::from(entry)));
            if location.is_err() {
                return Err(CompilerError::EntryNotValid(entry.to_owned()));
            }
            match self.compile_chunk(&location.unwrap(), &plugin_manager) {
                Ok(result) => {
                    parse_duration += result.duration;
                }
                Err(err) => {
                    eprintln!("{:?}", err);
                }
            }
        };


        return match self.emit(&options) {
            Ok(result) => {
                Ok(CompileResult {
                    overall_duration: start.elapsed(),
                    parse_duration,
                    emit_duration: result.duration,
                    emit_count: result.count
                })
            }
            Err(err) => Err(err)
        };
    }

    fn compile_chunk(&mut self, location: &ChunkLocation, plugin_manager: &PluginManager) -> Result<ParserResult, ParserError> {
        let id = location.generate_id();

        if self.chunk_map.contains_key(&id) {
            return Err(ParserError::new(ParserErrorKind::Syntax, format!("Chunk already exists"), SourceLocation::default()));
        }

        let file_name = location.path.file_name().unwrap().to_str().unwrap().to_owned();
        let mut chunk = Chunk::new(file_name, location.clone());

        chunk.load_source();

        let parse_result = self.parser.parse(ParserOptions::new(&mut chunk, &plugin_manager));
        return match parse_result {
            Ok(result) => {
                let required_chunks = self.resolver.resolve(&mut chunk, &result.requires).unwrap();
                chunk.syntax_tree = Some(result.syntax_tree.clone());

                for required_location in &required_chunks {
                    let required_id = required_location.generate_id();

                    self.compile_chunk(&required_location, plugin_manager);

                    let mut required_chunk = self.chunk_map.get_mut(&required_id).unwrap();
                    required_chunk.dependencies.used_by.push(id.clone());
                    chunk.dependencies.requires.push(required_id.clone());
                }

                self.push_chunk(chunk);

                Ok(result)
            }
            Err(err) => Err(err),
        };
    }

    fn emit<'a>(&'a self, options: &'a CompilerOptions) -> Result<EmitResult, CompilerError> {
        let start = Instant::now();
        let mut count = 0;

        if !options.out_dir.exists() {
            println!("{:?}", fs::create_dir_all(options.out_dir.to_str().unwrap()));
        }

        let generator = Generator {};

        for (_, chunk) in self.chunk_map.iter() {
            let out_path = Compiler::get_out_path(&chunk.location.path, &options.src_base_path, &options.out_dir);

            match chunk.syntax_tree {
                Some(ref node) => {
                    let src = generator.generate(node);

                    match fs::OpenOptions::new().create(true).write(true).open(out_path) {
                        Ok(ref file) => {
                            let mut buf = BufWriter::new(file);
                            buf.write_all(src.as_bytes());
                            count += 1;
                        },
                        Err(err) => {},
                    };
                }
                None => {}
            }
        };

        return Ok(EmitResult { count, duration: start.elapsed() });
    }

    fn get_out_path(file_path: &PathBuf, src_dir: &PathBuf, out_dir: &PathBuf) -> PathBuf {
        let mut i = 0;
        loop {
            if file_path.components().nth(i).eq(&src_dir.components().nth(i)) {
                i += 1;
            } else {
                break;
            }
        }

        let mut out_path = out_dir.clone();
        loop {
            match file_path.components().nth(i) {
                Some(item) => {
                    out_path.push(item.as_os_str());
                    i += 1;
                }
                None => break,
            }
        }

        return out_path;
    }
}
