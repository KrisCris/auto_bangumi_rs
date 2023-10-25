use auto_bangumi_rs::parser::Parser as BangumiParser;
use std::{
    fs::{self, create_dir_all},
    path::PathBuf,
};

use colored::Colorize;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[arg(short, long, value_name = "PATHS", num_args = 1..)]
    inputs: Vec<PathBuf>,
    #[arg(short, long, value_name = "DIRECTORY")]
    output: PathBuf,
    #[arg(short, long)]
    dryrun: bool,
    #[arg(short, long)]
    group_by_name: bool,
    #[command(subcommand)]
    mode: Mode,
}

#[derive(Subcommand)]
enum Mode {
    Move,
    Copy,
    HardLink,
}

fn main() {
    let cli = Cli::parse();
    let mut files = Vec::new();

    for bangumi_path in cli.inputs {
        if !bangumi_path.exists() {
            eprintln!(
                "Path {} does not exist!",
                bangumi_path.to_string_lossy().green()
            );
            continue;
        }

        match bangumi_path {
            _ if bangumi_path.is_file() => files.push(bangumi_path),
            _ if bangumi_path.is_dir() => {
                if let Ok(reader) = fs::read_dir(&bangumi_path) {
                    for entry in reader {
                        match entry {
                            Ok(file) => {
                                let path_buf = file.path();
                                if path_buf.is_file() {
                                    files.push(path_buf)
                                }
                            }
                            Err(e) => eprintln!(
                                "Error traversing directory {}, {}",
                                bangumi_path.to_string_lossy().green(),
                                e
                            ),
                        }
                    }
                }
            }
            _ => eprintln!(
                "Error occured with provided path {}",
                bangumi_path.to_string_lossy().green()
            ),
        }
    }

    for path in files {
        if let Some(bangumi) =
            BangumiParser::from_path(&path).and_then(|parser| parser.to_bangumi())
        {
            let out_path = bangumi.gen_fullpath(&cli.output, cli.group_by_name);
            match rename_file(&path, &out_path, &cli.mode, cli.dryrun) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            }
        } else {
            eprintln!(
                "Error parsing file: {}, skipping..",
                path.to_string_lossy().green()
            );
        }
    }
}

fn rename_file(
    src: &PathBuf,
    dst: &PathBuf,
    mode: &Mode,
    dryrun: bool,
) -> Result<(), std::io::Error> {
    println!(
        "- {} \n\t=> {}",
        src.to_string_lossy().bright_yellow(),
        dst.to_string_lossy().bright_blue(),
    );
    if dryrun {
        return Ok(());
    }

    if let Some(folder) = dst.parent() {
        if let Err(e) = create_dir_all(folder) {
            eprint!(
                "Error reanme file {}: {}",
                src.to_string_lossy().green(),
                e.to_string().red()
            )
        }
    }

    match mode {
        Mode::Move => fs::rename(src, dst),
        Mode::Copy => match fs::copy(src, dst) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        Mode::HardLink => fs::hard_link(src, dst),
    }
}

fn _sanitize_filename(filename: &str) -> String {
    // Characters invalid on Windows: < > : " / \ | ? *
    // Characters to avoid on Unix-like: / (as it's a directory separator) and null byte
    // Combining both, we get the pattern: [<>:\"/\\|?*\0]

    let sanitized: String = filename
        .chars()
        .filter(|&c| !"<>:\"/\\|?*\0".contains(c))
        .collect();

    sanitized
}
