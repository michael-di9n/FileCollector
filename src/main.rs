use std::{
    io,
    env, 
    fs::{self, DirEntry}, 
    process };

use std::path::{Path, PathBuf};

const POINT: &str = "├──";
const END_POINT: &str = "└──";
const BACKBONE: &str = "│";
const DEFAULT_OUTPUT_FOLDERNAME: &str = "default_out";

fn main() {

    let config =  Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}.");
        process::exit(1);
    });
    let prefix = String::new();

    create_output_folder(&config.output_folder);

    println!("{}", config.path);
    if let Err(e) =  visit_dirs(Path::new(&config.path), &display_metadata, &display_folder, prefix) {
        eprintln!("Error visiting directories {e}");
    } 
}

fn create_output_folder(output: &Option<String>) {
    match output {
        Some(folder_path) => {
            match fs::exists(folder_path) {
                Ok(_) => return,
                Err(_) => {
                    if let Err(e) = fs::create_dir(folder_path) {
                        eprint!("Encountered error creating output location {e}");
                    }
                }
            }
        },
        None => {
            match fs::exists(DEFAULT_OUTPUT_FOLDERNAME) {
                Ok(_) => return,
                Err(_) => {
                    if let Err(e) = fs::create_dir(DEFAULT_OUTPUT_FOLDERNAME) {
                        eprint!("Encountered error creating output location {e}");
                    }
                }
            }
        }
    }
}


fn display_metadata(dir_entry: &DirEntry, prefix: &String, is_end: bool) {
    // Display length and last accessed time
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
            let seperator = if end { " " } else { BACKBONE };
            // Modify the prefix if its the end then we should hand the prefix special
            if path.is_dir() {
                pathcb(&path, &prefix, end);
                visit_dirs(&path, cb, pathcb, prefix.clone() + seperator + "   ")?;
            } else {
                cb(&entry, &prefix, end);
            }
        }
    }
    Ok(())
}


struct Config {
    pub path: String,
    pub filename_pattern: Option<String>,
    pub filecont_pattern: Option<String>,
    pub output_folder: Option<String>,
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

        let mut filename_pattern: Option<String> = None;
        let mut filecont_pattern: Option<String> = None;
        let mut output_folder: Option<String> = None;

        while let Some(arg) = args.next()  {
            if arg == "-f" || arg == "-file" {
                filename_pattern = args.next();
            } 

            if arg == "-fc" || arg == "-fcontent" {
                filecont_pattern = args.next();
            }

            if arg == "-o" || arg == "-output" {
                output_folder = args.next();
            }
        }   

        Ok(Config {
            path,
            filename_pattern,
            filecont_pattern,
            output_folder,
        })
    }   
}
