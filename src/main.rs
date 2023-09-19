use std::{env, fs, path::Path, process::exit};

use auto_bangumi_rs::BangumiInfo;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ./program path_to_file");
        exit(1);
    }

    let path_str = &args[1];
    let path = Path::new(path_str);
    if !path.is_file() {
        eprintln!("Not a file!");
        exit(1);
    }

    if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
        let ext = match path.extension() {
            Some(p) => p.to_str().unwrap(),
            None => "",
        };

        match BangumiInfo::parse(filename) {
            Some(info) => {
                let new_filename = info.gen_filename(ext);
                println!("=> {}", new_filename);
                let new_path = path_str.replace(filename, &new_filename);

                match rename_file(&path_str, &new_path) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        exit(1);
                    }
                }
            }
            None => {
                eprintln!("You found a bug, report to me.");
                exit(1);
            }
        }
    }
}

fn rename_file(src: &str, dst: &str) -> Result<(), std::io::Error> {
    fs::rename(src, dst)
}
