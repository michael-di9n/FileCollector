use std::{
    env, 
    process };

use std::path::{Path};
use file_collector::Config;
use file_collector;
fn main() {

    let config =  Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}.");
        process::exit(1);
    });
    let prefix = String::new();

    file_collector::create_output_folder(&config.output_folder);

    println!("{}", config.path);
    if let Err(e) =  file_collector::visit_dirs(Path::new(&config.path), &file_collector::display_metadata, &file_collector::display_folder, prefix) {
        eprintln!("Error visiting directories {e}");
    } 
}