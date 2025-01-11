use std::{
    env, 
    process };

use std::path::Path;
use FileCollector::Config;
use FileCollector;
fn main() {

    let config =  Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}.");
        process::exit(1);
    });
    
    if config.path != None {
        let prefix = String::new();
        FileCollector::create_output_folder(&config.output_folder);
    
        println!("{}", config.path.as_ref().unwrap());
        if let Err(e) =  FileCollector::visit_dirs(Path::new(&config.path.as_ref().unwrap()), &config, &FileCollector::handle_file, &FileCollector::handle_folder, prefix) {
            eprintln!("Error visiting directories {e}");
        } 
    }
}