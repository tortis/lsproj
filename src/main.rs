use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;

const ROOT_SIGNATURES: [&str; 2] = [".git", "package.json"];

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    let root = args
        .get(1)
        .map(|v| v.to_string())
        .unwrap_or_else(|| std::env::var("HOME").unwrap());

    let projects = scan(root)?;
    println!("{}", projects.join("\n"));

    Ok(())
}

fn scan(root: String) -> Result<Vec<String>> {
    let mut stack = vec![PathBuf::from_str(&root).unwrap()];
    let mut projects = Vec::<String>::new();

    while let Some(p) = stack.pop() {
        if is_project(p.clone()) {
            let rel_path = p.to_str().unwrap().strip_prefix(&root).unwrap();
            projects.push(rel_path.into());
        } else {
            let dir = fs::read_dir(p)?;
            for entry in dir.filter_map(|e| e.ok()) {
                let is_dir = entry.file_type().map_or(false, |ft| ft.is_dir());
                if is_dir {
                    let entry_path = entry.path();
                    if entry_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| n.starts_with("."))
                        .unwrap_or(false)
                    {
                    } else {
                        stack.push(entry.path());
                    }
                }
            }
        }
    }

    Ok(projects)
}

fn is_project(p: PathBuf) -> bool {
    for sig in ROOT_SIGNATURES.iter() {
        if fs::exists(p.join(sig)).unwrap_or(false) {
            return true;
        }
    }

    false
}
