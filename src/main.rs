use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{anyhow, Result};

const ROOT_SIGNATURES: [&str; 3] = [".git", "package.json", ".lsproj"];

struct Options {
    root: String,
    max_depth: Option<usize>,
}

fn main() -> Result<()> {
    let Options { root, max_depth } = parse_args()?;

    let projects = scan(root, max_depth)?;
    println!("{}", projects.join("\n"));

    Ok(())
}

fn parse_args() -> Result<Options> {
    let args: Vec<_> = std::env::args().skip(1).collect();

    let mut root: Option<String> = None;
    let mut max_depth: Option<usize> = None;
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-d" | "--max-depth" => {
                i += 1;
                let value = args
                    .get(i)
                    .ok_or_else(|| anyhow!("{} requires a value", arg))?;
                max_depth = Some(
                    value
                        .parse()
                        .map_err(|_| anyhow!("invalid max depth: {}", value))?,
                );
            }
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            _ => {
                if arg.starts_with('-') {
                    return Err(anyhow!("unknown flag: {}", arg));
                }
                if root.is_some() {
                    return Err(anyhow!("unexpected argument: {}", arg));
                }
                root = Some(arg.clone());
            }
        }
        i += 1;
    }

    let root = root.unwrap_or_else(|| std::env::var("HOME").unwrap());

    Ok(Options { root, max_depth })
}

fn print_usage() {
    println!(
        "Usage: lsproj [ROOT] [-d N | --max-depth N]

Find all projects in a directory (a directory containing .git, package.json, or .lsproj).

Args:
  ROOT                    Directory to scan (defaults to $HOME)

Options:
  -d, --max-depth N       Limit scan depth; 1 only looks in ROOT itself,
                          2 scans ROOT and its immediate subdirectories, etc.
  -h, --help              Print this help"
    );
}

fn scan(root: String, max_depth: Option<usize>) -> Result<Vec<String>> {
    // (path, depth) where depth is the number of levels below root.
    let mut stack = vec![(PathBuf::from_str(&root).unwrap(), 0usize)];
    let mut projects = Vec::<String>::new();

    while let Some((p, depth)) = stack.pop() {
        if is_project(&p) {
            let rel_path = p.to_str().unwrap().strip_prefix(&root).unwrap();
            projects.push(rel_path.into());
        } else if max_depth.map_or(true, |max| depth < max) {
            let dir = match fs::read_dir(&p) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Skipping {}: {}", p.display(), e);
                    continue;
                }
            };
            for entry in dir.filter_map(|e| e.ok()) {
                let is_dir = entry.file_type().map_or(false, |ft| ft.is_dir());
                if is_dir {
                    let entry_path = entry.path();
                    let hidden = entry_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| n.starts_with("."))
                        .unwrap_or(false);
                    if !hidden {
                        stack.push((entry_path, depth + 1));
                    }
                }
            }
        }
    }

    Ok(projects)
}

fn is_project(p: &PathBuf) -> bool {
    for sig in ROOT_SIGNATURES.iter() {
        if fs::exists(p.join(sig)).unwrap_or(false) {
            return true;
        }
    }

    false
}
