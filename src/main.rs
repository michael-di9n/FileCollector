use std::{
    io,
    env, 
    fs::{self, DirEntry}, 
    process };

use std::path::{Path, PathBuf};

const POINT: &str = "├──";
const END_POINT: &str = "└──";
const BACKBONE: &str = "│";

fn main() {

    let config =  Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}.");
        process::exit(1);
    });

    println!("visiting files in path: {}", config.path);
    let prefix = String::new();
    if let Err(e) =  visit_dirs(Path::new(&config.path), &display_metadata, &display_folder, prefix) {
        eprintln!("Error visiting directories {e}");
    } 
}


fn display_metadata(dir_entry: &DirEntry, prefix: &String, is_end: bool) {
    let point = if is_end { END_POINT } else { POINT };
    println!("{prefix}{}{:?}", point, dir_entry.file_name())
}

fn display_folder(folder: &PathBuf, prefix: &String, is_end: bool) {
    let point = if is_end { END_POINT } else { POINT };
    match folder.file_name() {
        Some(name) => println!("{prefix}{}{:?}", point, name),
        None => return
    }
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry, &String, bool), pathcb: &dyn Fn(&PathBuf, &String, bool), prefix: String) -> io::Result<()> {
    if dir.is_dir() {
        // let mut later_path: Vec<PathBuf> = Vec::new();
        let dir_vec: Vec<DirEntry> = fs::read_dir(dir)?.map(|result| result.unwrap()).collect(); // TODO handle unwrap here
        
        for i in 0..dir_vec.len() {
            let entry = &dir_vec[i];
            let path = entry.path();
            let end = i == dir_vec.len() - 1;
            if path.is_dir() {
                pathcb(&path, &prefix, end);
                visit_dirs(&path, cb, pathcb, prefix.clone() + BACKBONE + "   ")?;
            } else {
                cb(&entry, &prefix, end);
            }
        }
    }
    Ok(())
}


struct Config {
    pub path: String,
}


impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let path = match args.next() {
            Some(path) => path,
            None => return Err("Didn't get a path string")
        };

        Ok(Config {
            path
        })
    }   
}
