use std::env;

use git2::{Repository, Tree, Error};

fn commits_for_subdir(repo_path: &str, subdir: &str) -> Result<(), Error> {
    log::info!("commits_for_subdir:+ repo_path: {repo_path}, subdir: {subdir}");
    let repo = Repository::open(repo_path)?;
    let mut revwalk = repo.revwalk()?;

    // Push HEAD to start traversal
    revwalk.push_head()?;

    // Previous commit's tree
    let mut prev_tree: Option<Tree> = None;

    for oid_result in revwalk {
        log::info!("commits_for_subdir: TOL prev_tree: {prev_tree:?}, oid_result: {oid_result:?}");
        let oid = oid_result?;
        let commit = repo.find_commit(oid)?;
        log::info!("commits_for_subdir: oid {oid:?}, commit: {commit:?}");

        // Get the tree of the current commit
        let tree = commit.tree()?;

        if let Some(prev) = prev_tree.as_ref() {
            // Compare the current tree with the previous one
            let diff = repo.diff_tree_to_tree(Some(prev), Some(&tree), None)?;

            let mut found = false;
            diff.foreach(
                &mut |delta, _| {
                    if let Some(path) = delta.old_file().path() {
                        if path.starts_with(subdir) {
                            log::info!("commits_for_subdir: old_file starts with subidr, old_file: {path:?}");
                            found = true;
                        } else {
                            log::info!("commits_for_subdir: !subdir old_file: {path:?}");
                        }
                    }
                    if let Some(path) = delta.new_file().path() {
                        if path.starts_with(subdir) {
                            log::info!("commits_for_subdir: new_file starts with subdir new_file: {path:?}");
                            found = true;
                        } else {
                            log::info!("commits_for_subdir: !subdir new_file(): {path:?}");
                        }
                    }
                    if found {
                        // Stop further diff processing if we find a match
                        log::info!("commits_for_subdir: found stop processing");
                        return false;
                    }
                    true
                },
                None,
                None,
                None,
            )?;

            if found {
                println!("{}: {}", commit.id(), commit.summary().unwrap_or("No summary"));
            }
        } else {
            // For the first commit, just check if the subdir exists in its tree
            if tree.get_path(std::path::Path::new(subdir)).is_ok() {
                println!("{}: {}", commit.id(), commit.summary().unwrap_or("No summary"));
            }
        }

        // Update the previous tree
        prev_tree = Some(tree);
        log::info!("commits_for_subdir: BOL new prev_tree: {prev_tree:?}");
    }

    log::info!("commits_for_subdir:- repo_path: {repo_path}, subdir: {subdir})");
    Ok(())
}

fn usage() {
    eprintln!("Usage: {} <repo_path> <subdir>", env::args().next().unwrap());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    custom_logger::env_logger_init("info");

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
