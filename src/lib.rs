
use std::{
    io,
    fs::{self, DirEntry}, 
    process };

use std::path::{Path, PathBuf};
const POINT: &str = "├──";
const END_POINT: &str = "└──";
const BACKBONE: &str = "│";
const DEFAULT_OUTPUT_FOLDERNAME: &str = "default_out";


pub fn create_output_folder(folder: &Option<String>) {
    match folder {
        Some(folder_path) => {
            let exist = fs::exists(folder_path);
            let _ = match exist {
                    Ok(false) => fs::create_dir(folder_path),
                    Ok(true) => return,
                    Err(e) => { println!("Error checking if {folder_path} exist {e}"); Err(e)},
            };
        },
        None => {
            if let Ok(false) = fs::exists(DEFAULT_OUTPUT_FOLDERNAME) {
                if let Err(e) = fs::create_dir(DEFAULT_OUTPUT_FOLDERNAME) {
                    eprint!("Encountered error creating output location {e}");
                }
            }
        }
    }
}


pub fn display_metadata(dir_entry: &DirEntry, prefix: &String, is_end: bool) {
    // Display length and last accessed time
    let point = if is_end { END_POINT } else { POINT };
    println!("{prefix}{}{:?}", point, dir_entry.file_name())
}

pub fn display_folder(folder: &PathBuf, prefix: &String, is_end: bool) {
    let point = if is_end { END_POINT } else { POINT };
    match folder.file_name() {
        Some(name) => println!("{prefix}{}{:?}", point, name),
        None => return
    }
}

pub fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry, &String, bool), pathcb: &dyn Fn(&PathBuf, &String, bool), prefix: String) -> io::Result<()> {
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


pub struct Config {
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
            None => return Err("Didn't get a path string. Use -h or -help to get information")
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

            if arg == "-h" || arg == "-help" {
                println!("example file_collector <path> [-f <filenamepattern>] [-fc <filecontent pattern>] [-o <output folder>]")
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

#[cfg(test)]
mod tests {
    use fs::remove_dir;

    use super::*;

    #[test]
    fn file_output_some() {
        let folder_name = Some(String::from("foldera"));
        
        let _ = fs::remove_dir("foldera");
        create_output_folder(&folder_name);
        assert!(fs::exists(folder_name.unwrap()).unwrap());
        let _ = fs::remove_dir("foldera");
    }

    #[test]
    fn file_output_none() {
        create_output_folder(&None);
        assert!(fs::exists(DEFAULT_OUTPUT_FOLDERNAME).unwrap());
        let _ = fs::remove_dir(DEFAULT_OUTPUT_FOLDERNAME);

    }

    #[test]
    fn create_folder() {
        let _ = fs::create_dir("CREATED");
        assert!(fs::exists("CREATED").unwrap());
        let _ = fs::remove_dir("CREATED");
    }
}