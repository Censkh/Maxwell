mod compiler;
mod plugins;

use std::time::Duration;

use compiler::transform::PluginManager;

//use plugins::{MinifyPlugin, PrettifyPlugin};

use compiler::Compiler;

fn main() {
    let mut compiler = Compiler::new();

    let plugins = PluginManager::new();
    //plugins.add_plugin(MinifyPlugin {});

    match compiler.compile(plugins) {
        Ok(output) => {
            println!("Compiled {} chunks in {}:", output.emit_count, duration_to_string(output.overall_duration));
            println!("   Parse took {}", duration_to_string(output.parse_duration));
            println!("   Emit took {}", duration_to_string(output.emit_duration));
        }
        Err(err) => {
            println!("Compiler error: \n{}", err.to_string());
        }
    }
}

fn duration_to_string(duration: Duration) -> String {
    let mut time = duration.as_secs() as f64 / 1_000.0;
    time += duration.subsec_nanos() as f64 / 1_000_000.0;
    return format!("{} ms", time);
}