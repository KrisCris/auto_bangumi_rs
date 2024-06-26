use auto_bangumi_rs::parser::Parser as BangumiParser;
use std::{
    fs::{self, create_dir_all},
    path::{Path, PathBuf},
};

use colored::Colorize;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about="A bangumi (anime) renamer authored by _connlost.", long_about = None, arg_required_else_help = true)]
struct Cli {
    #[arg(short, long, value_name = "PATH", help = "Either a file or the parent directory")]
    input: Vec<PathBuf>,
    #[arg(short, long, value_name = "DIRECTORY", help = "Optional, default to the parent directory of the renamed file")]
    output: Option<PathBuf>,
    #[arg(short, long)]
    dryrun: bool,
    #[arg(short, long, help = "Group animes by series and season")]
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

fn collect_files(paths: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for bangumi_path in paths {
        if !bangumi_path.exists() {
            eprintln!(
                "Path {} does not exist!",
                bangumi_path.to_string_lossy().green()
            );
            continue;
        }

        match bangumi_path {
            _ if bangumi_path.is_file() => files.push(bangumi_path.to_owned()),
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

    files
}

fn try_read_season_from_dir(parent: &Path) -> Option<u32> {
    let dot_season_path = parent.join(".season");
    return fs::read_to_string(dot_season_path)
        .ok()?
        .trim()
        .parse::<u32>()
        .ok();
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
    let sanitized: String = filename
        .chars()
        .filter(|&c| !"<>:\"/\\|?*\0".contains(c))
        .collect();

    sanitized
}

fn process_files(paths: Vec<PathBuf>, cli: &Cli) {
    for path in paths {
        let season = path.parent().and_then(try_read_season_from_dir);
        if let Some(bangumi) =
            BangumiParser::from_path(&path).and_then(|parser| parser.to_bangumi(season))
        {
            let output_path = match &cli.output {
                Some(output) => output.to_owned(),
                None => path.parent().unwrap().to_path_buf(),
            };

            let out_path = bangumi.gen_fullpath(&output_path, cli.group_by_name);
            match rename_file(&path, &out_path, &cli.mode, cli.dryrun) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            }
        } else {
            eprintln!("Skipping {}", path.to_string_lossy().green());
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let files = collect_files(&cli.input);
    process_files(files, &cli);
}
