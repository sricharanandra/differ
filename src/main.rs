use std::error::Error;
use walkdir::{DirEntry, WalkDir};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Searching for .git directories, ignoring hidden directories");

    let git_dirs: Vec<String> = WalkDir::new("/home")
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| !is_hidden_dir(entry))
        .filter(is_git_dir)
        .map(|entry| entry.path().display().to_string())
        .collect();

    if git_dirs.is_empty() {
        println!("No .git directories found.");
    } else {
        println!("Found the following .git directories:");
        for dir in &git_dirs {
            println!("{}", dir);
        }
    }

    Ok(())
}

fn is_git_dir(entry: &DirEntry) -> bool {
    entry.path().is_dir()
        && entry
            .path()
            .file_name()
            .and_then(|name| name.to_str())
            .map_or(false, |name| name == ".git")
}

fn is_hidden_dir(entry: &DirEntry) -> bool {
    entry.path().ancestors().skip(1).any(|ancestor| {
        ancestor
            .file_name()
            .and_then(|name| name.to_str())
            .map_or(false, |s| s.starts_with("."))
    })
}

