
use std::{
    io,
    fs::{self, DirEntry}};
use regex::Regex;
use sha2::{Sha256, Digest};

const VERSION: &str = "0.1.0 beta";
use std::path::{Path, PathBuf};
const POINT: &str = "├──";
const END_POINT: &str = "└──";
const BACKBONE: &str = "│";
const DEFAULT_OUTPUT_FOLDERNAME: &str = "default_out";


pub fn create_output_folder(folder_path: &String) {
        let exist = fs::exists(folder_path);
        let _ = match exist {
                Ok(false) => fs::create_dir(folder_path),
                Ok(true) => return,
                Err(e) => { println!("Error checking if {folder_path} exist {e}"); Err(e)},
        };
}


pub fn handle_file(dir_entry: &DirEntry, prefix: &String, is_end: bool, config: &Config) {
    let point = if is_end { END_POINT } else { POINT };
    let filename = dir_entry.file_name();
    if let Some(filename_re) = &config.filename_pattern {
        if filename_re.is_match(filename.to_str().unwrap()) {
            print!("{prefix}{}{:?}", point, filename);
            let base = PathBuf::from(config.output_folder.clone());
            let full_path = base.join(filename.to_str().unwrap());
            println!(" (!) Found A Match!");

            if let Err(e) = fs::copy(&dir_entry.path(), &full_path) {
                println!("Encontered error when copying {:?} {e}", dir_entry.path());
            }
            if config.should_move {
                if let Err(e) = fs::remove_file(&full_path) {
                    eprintln!("Encountered error removing file {e}");
                }
            }
            
        } else {
            println!("{prefix}{}{:?}", point, dir_entry.file_name());
        }
    } else {
        println!("{prefix}{}{:?}", point, dir_entry.file_name());
    }
}

pub fn handle_folder(folder: &PathBuf, prefix: &String, is_end: bool, config: &Config) {
    let point = if is_end { END_POINT } else { POINT };
    match folder.file_name() {
        Some(name) =>  {
            if let Some(filename_re) = &config.filename_pattern {
                if filename_re.is_match(name.to_str().unwrap()) {
                    println!("{prefix}{}{:?} (Match)", point, name);
                } else {
                    println!("{prefix}{}{:?}", point, name);                    
                }
            } else {
                println!("{prefix}{}{:?}", point, name);
            }
        },
        None => return
    }
}

pub fn visit_dirs(dir: &Path, config: &Config, cb: &dyn Fn(&DirEntry, &String, bool, &Config), pathcb: &dyn Fn(&PathBuf, &String, bool, &Config), prefix: String) -> io::Result<()> {
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
                pathcb(&path, &prefix, end, config);
                visit_dirs(&path, &config, cb, pathcb, prefix.clone() + seperator + "   ")?;
            } else {
                cb(&entry, &prefix, end, config);
            }
        }
    }
    Ok(())
}


pub struct Config {
    pub path: Option<String>,
    pub filename_pattern: Option<Regex>,
    pub filecont_pattern: Option<Regex>,
    pub output_folder: String,
    pub should_move: bool
}


impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        let _ = args.next();

        let mut filename_pattern: Option<Regex> = None;
        let mut filecont_pattern: Option<Regex> = None;
        let mut output_folder: String = DEFAULT_OUTPUT_FOLDERNAME.to_owned();
        let mut should_move = false;
        let mut path = None;
        let mut had_arg = false;
        while let Some(arg) = args.next()  {
            had_arg = true;

            if arg == "-p" || arg == "--path" {
                path = args.next();
            }

            if arg == "-f" || arg == "--file" {
                filename_pattern = match args.next() {
                    Some(pattern) => {
                        match Regex::new(&pattern) {
                            Ok(re) => Some(re),
                            Err(e) => {
                                println!("Tried to convert to regex (file name): {pattern}. encountered error: {e}");
                                None
                            }
                        }
                    }
                    None => None,
                }
            }

            if arg == "-fc" || arg == "--fcontent" {
                filecont_pattern = match args.next()  {
                    Some(pattern) => {
                        match Regex::new(&pattern) {
                            Ok(re) => Some(re),
                            Err(e) => {
                                println!("Tried to convert to regex (file contents): {pattern}. encountered error: {e}");
                                None
                            }
                        }
                    }
                    None => None,
                }
            }

            if arg == "-o" || arg == "--output" {
                if let Some(out_foldername) =  args.next() {
                    output_folder = out_foldername;
                }
            }

            if arg == "-h" || arg == "--help" {
                println!("example file_collector [-p|--path <path>] [-f|--filename <filenamepattern>] [-o|--output <output folder>]")
                // println!("example file_collector <path> [-f <filenamepattern>] [-fc <filecontent pattern>] [-o <output folder>] [-move ]")
            }
            if arg == "move" {
                should_move = true;
                println!("moving file to output instead of copy")
            }

            if arg == "-v" || arg == "--version" {
                println!("{VERSION}");
            }
        }

        if !had_arg {
            println!("File Collector is a light weight file traversal and collection program use the following syntax\nFileCollector.exe [-p|--path <path>] [-f <filenamepattern>] [-o <output folder>]")
        }   

        Ok(Config {
            path,
            filename_pattern,
            filecont_pattern,
            output_folder,
            should_move
        })
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_output_some() {
        let folder_name = Some(String::from("foldera"));
        
        let _ = fs::remove_dir("foldera");
        create_output_folder(&String::from("foldera"));
        assert!(fs::exists(folder_name.unwrap()).unwrap());
        let _ = fs::remove_dir("foldera");
    }

    #[test]
    fn create_folder() {
        let _ = fs::create_dir("CREATED");
        assert!(fs::exists("CREATED").unwrap());
        let _ = fs::remove_dir("CREATED");
    }

    #[test]
    fn digest_test() {
        let base = PathBuf::from("randompath/");
        let mut hasher = Sha256::new();
        hasher.update(base.to_str().unwrap().as_bytes());
        let hash = hasher.finalize();
        println!("Hash result{:x}", hash);
    }


}