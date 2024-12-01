use std::env;

use git2::{Repository, Error, Tree, DiffOptions, Oid, Commit};

//fn commits_for_subdir(repo_path: &str, subdir: &str) -> Result<(), Error> {
//    log::info!("commits_for_subdir:+ repo_path: {repo_path}, subdir: {subdir}");
//    let repo = Repository::open(repo_path)?;
//    let mut revwalk = repo.revwalk()?;
//
//    // Push HEAD to start traversal
//    revwalk.push_head()?;
//
//    // Sort commits by in topological order. This is different from the default,
//    // which is git2::Sort::NONE. With NONE the order is unspecified and may change
//    // according the comment for NONE in git2::Sort.
//    revwalk.set_sorting(git2::Sort::TOPOLOGICAL).unwrap();
//
//    let subdir_is_root = subdir == "/";
//    log::info!("subdir_is_root: {subdir_is_root}");
//
//    for oid_result in revwalk {
//        let oid = oid_result?;
//        let commit = repo.find_commit(oid)?;
//        log::info!("Processing commit: {}", commit.id());
//
//        // Get the tree of the current commit
//        let tree = commit.tree()?;
//        log::info!("Tree: {}", tree.id());
//
//        if subdir_is_root {
//            // Always print for root
//            println!("{}: {}", commit.id(), commit.summary().unwrap_or("No summary"));
//        } else if tree.get_path(std::path::Path::new(subdir)).is_ok() {
//            // Print only if the subdirectory exists
//            println!("{}: {}", commit.id(), commit.summary().unwrap_or("No summary"));
//        }
//    }
//
//    log::info!("commits_for_subdir:- repo_path: {repo_path}, subdir: {subdir}");
//    Ok(())
//}

fn commits_for_subdir(repo_path: &str, subdir: &str) -> Result<(), Error> {
    log::info!("commits_for_subdir:+ repo_path: {repo_path}, subdir: {subdir}");
    let repo = Repository::open(repo_path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    for oid_result in revwalk {
        let oid: Oid = oid_result?;
        let commit: Commit = repo.find_commit(oid)?;
        let tree: Tree = commit.tree()?;
        log::info!("Processing commit: id: {} tree: {tree:?}", commit.id());

        let parent_tree: Option<Tree> = if let Some(parent) = commit.parents().next() {
            Some(parent.tree()?)
        } else {
            None
        };
        log::info!("Processing commit: parent_tree: {parent_tree:?}");

        // Move outside of loop?
        let mut diff_opts = DiffOptions::new();
        diff_opts.pathspec(subdir);

        let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), Some(&mut diff_opts))?;
        log::info!("Processing commit: diff.deltas().len(): {}", diff.deltas().len());
        if diff.deltas().len() > 0 {
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
