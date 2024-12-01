use std::{collections::HashMap, env};

use git2::{Repository, Error, Tree, DiffOptions, Oid, Commit};

fn commits_for_subdir(repo_path: &str, subdir: &str) -> Result<(), Error> {
    log::info!("commits_for_subdir:+ repo_path: {repo_path}, subdir: {subdir}");
    let repo = Repository::open(repo_path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    // Sort commits by in topological order, by using this the associated
    // commits of a "Merge pull request" precedes the associated commits.
    // I also tried adding git2::Sort::TIME, but the commits were not in
    // the expected order.
    //
    // For example with a local checkout of [rp-hal](https://github.com/rp-rs/rp-hal)
    // and `revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;` a command from rp-hal root
    // `iterate-over-subdir . rp235x-hal > iterate-over-subdir-topo.txt`
    // the order was:
    //   960afaf3cf6015f34334d03420391432dc8bc0c2: Merge pull request #841 from jannic/update-rp2350-uart
    //   f84b76497648acb355ca5ca0baa1b760a3f337c0: Add uart_loopback example.
    //   84b1753b4ed87ef65c5df77b2de73f8d0e6b3642: Port UART updates to rp235x-hal
    //   2d815bf10790a0ddbe436a6ac9d2e6693bf742b1: Merge pull request #842 from jannic/update-rp2350-spi
    //   1adf2b4ae186bbff19a1c9ee1d613f6ef0d2b031: Port SPI changes from rp2040-hal to rp235x-hal
    //
    // But with `revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::TIME)?;`
    // with command `iterate-over-subdir . rp235x-hal > iterate-over-subdir-topo-time.txt`
    // the order was:
    //   960afaf3cf6015f34334d03420391432dc8bc0c2: Merge pull request #841 from jannic/update-rp2350-uart
    //   f84b76497648acb355ca5ca0baa1b760a3f337c0: Add uart_loopback example.
    //   2d815bf10790a0ddbe436a6ac9d2e6693bf742b1: Merge pull request #842 from jannic/update-rp2350-spi
    //   1adf2b4ae186bbff19a1c9ee1d613f6ef0d2b031: Port SPI changes from rp2040-hal to rp235x-hal
    //   84b1753b4ed87ef65c5df77b2de73f8d0e6b3642: Port UART updates to rp235x-hal
    //
    // And you can see the the 84b175 commit is "wrong" as it's parent is 960afaf.
    //revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::Time)?; // wrong order
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;

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

        let parent_ids: Vec<Oid> = commit.parent_ids().collect();
        log::info!("Processing commit: parent_ids: {parent_ids:?}");

        //let child_ids: Vec<Oid> = commit.c().collect();
        //log::info!("Processing commit: parent_ids: {parent_ids:?}");

        // Move outside of loop?
        let mut diff_opts = DiffOptions::new();
        diff_opts.pathspec(subdir);

        let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), Some(&mut diff_opts))?;
        log::info!("Processing commit: diff.deltas().len(): {}", diff.deltas().len());
        if diff.deltas().len() > 0 {
            let parent_ids: Vec<Oid> = commit.parent_ids().collect();
            println!("{}: {} -- parent_ids: {parent_ids:?}", commit.id(), commit.summary().unwrap_or("No summary"));
        }
    }

    log::info!("commits_for_subdir:- repo_path: {repo_path}, subdir: {subdir}");
    Ok(())
}

fn commit_relationships(repo_path: &str) -> Result<(), Error> {
    println!("commit_relationships:+ repo_path: {}", repo_path);
    let repo = Repository::open(repo_path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;

    // Map to track parent-to-children relationships
    let mut child_map: HashMap<String, Vec<String>> = HashMap::new();

    println!("\ncommit_relationships: print id, summary and parent_ids:");
    for oid_result in revwalk {
        let oid = oid_result?;
        let commit = repo.find_commit(oid)?;

        for parent_id in commit.parent_ids() {
            let parent_id_str = parent_id.to_string();
            child_map
                .entry(parent_id_str)
                .or_default()
                .push(commit.id().to_string());
        }

        println!(
            "{}: {} -- parent_ids: {:?}",
            commit.id(),
            commit.summary().unwrap_or("No summary"),
            commit.parent_ids().map(|id| id.to_string()).collect::<Vec<_>>()
        );
    }

    println!("\ncommit_relationships: print the child map:");
    // Print the child map
    for (parent, children) in &child_map {
        println!("Parent {} has children: {:?}", parent, children);
    }

    println!("commit_relationships:- repo_path: {}", repo_path);
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

    commit_relationships(&repo_path)?;

    log::info!("main:-");
    Ok(())
}
