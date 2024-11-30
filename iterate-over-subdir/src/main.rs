use std::env;

use git2::{Repository, Error};

fn commits_for_subdir(repo_path: &str, subdir: &str) -> Result<(), Error> {
    log::info!("commits_for_subdir:+ repo_path: {repo_path}, subdir: {subdir}");
    let repo = Repository::open(repo_path)?;
    let mut revwalk = repo.revwalk()?;

    // Push HEAD to start traversal
    revwalk.push_head()?;

    for oid_result in revwalk {
        let oid = oid_result?;
        let commit = repo.find_commit(oid)?;
        log::info!("Processing commit: {}", commit.id());

        // Get the tree of the current commit
        let tree = commit.tree()?;
        log::info!("Tree: {}", tree.id());

        // Check if the subdirectory exists in this tree
        if tree.get_path(std::path::Path::new(subdir)).is_ok() {
            println!("{}: {}", commit.id(), commit.summary().unwrap_or("No summary"));
        }
    }

    log::info!("commits_for_subdir:- repo_path: {repo_path}, subdir: {subdir}");
    Ok(())
}

fn usage() {
    eprintln!("Usage: {} <repo_path> <subdir>", env::args().next().unwrap());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    custom_logger::env_logger_init("none");

    log::info!("main:+");
    let repo_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            usage();
            return Err("Repository path not provided".into());
        }
    };
    let subdir_path = match env::args().nth(2) {
        Some(path) => path,
        None => {
            usage();
            return Err("Subdirectory path not provided".into());
        }
    };
    if let Err(e) = commits_for_subdir(&repo_path, &subdir_path) {
        eprintln!("Error: {e}");
        return Err(e.into());
    }

    log::info!("main:-");
    Ok(())
}
