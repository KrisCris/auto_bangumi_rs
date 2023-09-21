use auto_bangumi_rs::parser::Parser;
use std::{env, fs, path::Path, process::exit};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: ./program file1 file2 file3...");
        exit(1);
    }

    args.remove(0);
    for arg in args {
        let path_str = &arg;
        let path = Path::new(path_str);
        if !path.is_file() {
            eprintln!("Not a file!");
            continue;
        }

        if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
            let ext = match path.extension() {
                Some(p) => p.to_str().unwrap(),
                None => "",
            };
            let parser = Parser::new(filename.to_owned());
            match parser.to_bangumi() {
                Some(info) => {
                    let new_filename = sanitize_filename(&info.gen_filename(ext));
                    println!("=> {}", new_filename);
                    let new_path = path_str.replace(filename, &new_filename);

                    match rename_file(&path_str, &new_path) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            continue;
                        }
                    }
                }
                None => {
                    eprintln!("You found a bug, report to me.");
                    continue;
                }
            }
        }
    }
}

fn rename_file(src: &str, dst: &str) -> Result<(), std::io::Error> {
    fs::rename(src, dst)
}

fn sanitize_filename(filename: &str) -> String {
    // Characters invalid on Windows: < > : " / \ | ? *
    // Characters to avoid on Unix-like: / (as it's a directory separator) and null byte
    // Combining both, we get the pattern: [<>:\"/\\|?*\0]

    let sanitized: String = filename
        .chars()
        .filter(|&c| !"<>:\"/\\|?*\0".contains(c))
        .collect();

    sanitized
}
