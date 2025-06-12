use std::fs;
use std::io::{stdin, stdout, Result, Write};
use std::time::Instant;
use std::path::{PathBuf};

fn get_input(query: &str) -> Result<String> {
    println!("{}", query);
    stdout().flush()?;

    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}
    
fn organize_dir(dir_path: PathBuf) {
    if !dir_path.exists() {
        eprintln!("Directory does not exist: '{}'", dir_path.display());
        return;
    } else {
        let dir_files = match std::fs::read_dir(&dir_path) {
            Ok(dir_files) => dir_files,
            Err(e) => {
                eprintln!("Error reading directory '{}': '{}'", dir_path.display(), e);
                return;
            },
        };

        for file in dir_files {
            if let Ok(file) = file {
                if file.path().is_dir() {
                    organize_dir(file.path());
                    // println!("Skipping directory: '{}'", file.path().display());
                    // continue;
                }

                let file_extension = match file.path().extension() {
                    None => {
                        println!("Skipping file without extension: '{}'", file.path().display());
                        continue;
                    },
                    Some(ext) => match ext.to_str() {
                        Some(s) => s.to_lowercase(),
                        None => {
                            println!("Skipping file with non-UTF8 extension: '{}'", file.path().display());
                            continue;
                        },
                    },
                };

                let ext_dir = PathBuf::from(dir_path.join(file_extension));
                create_dir_if_not_exists(&ext_dir);
                move_file(&file.path(), &ext_dir.join(file.file_name()));

                }
            }
        }
    }

    fn create_dir_if_not_exists(dir_path: &PathBuf) {
        if !dir_path.exists() {
           if let Err(e) = fs::create_dir(dir_path) {
                eprintln!("Error creating directory '{}': '{}'", dir_path.display(), e);
            }
        } 
    }

    fn  move_file(from: &PathBuf, to: &PathBuf) {
        if let Err(e) = fs::rename(from, to) {
            eprintln!("Error moving file from '{}' to '{}': '{}'", from.display(), to.display(), e);
        }
    }

fn main() {
    loop {
        let dir_path = match get_input("Enter the path to the dir you want to organize") {
            Ok(dir_path) => dir_path,
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            },
        };

        let now = Instant::now();
        organize_dir(PathBuf::from(dir_path));
        println!("Organized directory in {} seconds", now.elapsed().as_secs_f64());
    }
}
