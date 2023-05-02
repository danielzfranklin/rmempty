use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Remove empty directories recursively
struct Args {
    /// Directory to look inside (not removed itself)
    #[arg()]
    dir: String,

    /// Print directories that would be removed
    #[arg(short, long)]
    dry_run: bool,
}

fn main() {
    let Args { dir, dry_run } = Args::parse();

    let dir = PathBuf::from(dir);
    if !dir.is_dir() {
        eprintln!("{} is not a directory", dir.display());
        std::process::exit(1);
    }

    eprintln!("Scanning {}", dir.display());
    let mut to_remove = Vec::new();
    scan(&dir, &mut to_remove);

    eprint!("\nRemoving");
    if dry_run {
        eprint!(" (dry run)");
    }
    eprintln!(":");

    for path in &to_remove {
        println!("{}", path.display());

        if !dry_run {
            if let Err(err) = fs::remove_dir(path) {
                eprintln!("\tError: {err}");
            }
        }
    }
}

fn scan(dir: &Path, to_remove: &mut Vec<PathBuf>) {
    let mut remove_dir = true;

    for entry in fs::read_dir(dir).expect("read_dir") {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Error reading {}: {err}", dir.display());
                continue;
            }
        };
        let ty = match entry.file_type() {
            Ok(ty) => ty,
            Err(err) => {
                eprintln!("Error reading {}: {err}", entry.path().display());
                continue;
            }
        };

        if ty.is_dir() {
            let path = entry.path();
            scan(&path, to_remove);
            if to_remove.last() != Some(&path) {
                remove_dir = false;
            }
        } else {
            remove_dir = false;
        }
    }

    if remove_dir {
        to_remove.push(dir.to_owned());
    }
}
