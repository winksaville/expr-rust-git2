use std::env;

use git2::{Error, Repository};

fn commits_for_subdir(repo_path: &str, subdir: &str) -> Result<(), Error> {
    log::info!("commits_for_subdir:+ repo_path: {repo_path}, subdir: {subdir}");
    let repo = Repository::open(repo_path)?;
    let mut revwalk = repo.revwalk()?;

    // Push HEAD to start traversal
    revwalk.push_head()?;

    // Sort commits by in topological order. This is different from the default,
    // which is git2::Sort::NONE. With NONE the order is unspecified and may change
    // according the comment for NONE in git2::Sort.
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL).unwrap();

    let subdir_is_root = subdir == "/";
    log::info!("subdir_is_root: {subdir_is_root}");

    for oid_result in revwalk {
        let oid = oid_result?;
        let commit = repo.find_commit(oid)?;
        log::info!("Processing commit: {}", commit.id());

        // Get the tree of the current commit
        let tree = commit.tree()?;
        log::info!("Tree: {}", tree.id());

        if subdir_is_root {
            // Always print for root
            println!(
                "{}: {}",
                commit.id(),
                commit.summary().unwrap_or("No summary")
            );
        } else if tree.get_path(std::path::Path::new(subdir)).is_ok() {
            // Print only if the subdirectory exists
            println!(
                "{}: {}",
                commit.id(),
                commit.summary().unwrap_or("No summary")
            );
        }
    }

    log::info!("commits_for_subdir:- repo_path: {repo_path}, subdir: {subdir}");
    Ok(())
}

fn usage() {
    eprintln!(
        "Usage: {} <repo_path> <subdir>",
        env::args().next().unwrap()
    );
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
