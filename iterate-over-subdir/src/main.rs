use std::env;

use git2::{Commit, DiffOptions, Error, Oid, Repository, Tree};

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

    let mut last_pr_commit_oid: Option<Oid> = None;
    for oid_result in revwalk {
        let oid: Oid = oid_result?;
        if Some(oid) == last_pr_commit_oid {
            log::info!(
                "Processing commit: {oid:?} - Skipping last commit of previous Merge pull request"
            );
            continue;
        }
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

        // Move outside of loop?
        let mut diff_opts = DiffOptions::new();
        diff_opts.pathspec(subdir);

        let diff =
            repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), Some(&mut diff_opts))?;
        log::info!(
            "Processing commit: diff.deltas().len(): {}",
            diff.deltas().len()
        );
        if diff.deltas().len() > 0 {
            println!("{}: {}", oid, commit.summary().unwrap_or("No summary"));
            if commit.parent_count() > 1 {
                last_pr_commit_oid = print_merge_commits(&repo, oid)?;
                log::info!("last_pr_commit_oid: {last_pr_commit_oid:?}",);
                if last_pr_commit_oid.is_none() {
                    // This is a root commit
                    println!("  - Hit a root commit");
                    break;
                }
            }
        }
    }

    log::info!("commits_for_subdir:- repo_path: {repo_path}, subdir: {subdir}");
    Ok(())
}

//fn collect_merge_commits(repo: &Repository, merge_commit_oid: Oid) -> Result<Vec<Commit>, Error> {
// Print merge commits and return the last commit in the PR branch
// if last commit is a root commit ???????????????
fn print_merge_commits(repo: &Repository, merge_commit_oid: Oid) -> Result<Option<Oid>, Error> {
    log::info!("print_merge_commits:+ merge_commit_oid: {merge_commit_oid}");
    let merge_commit = repo.find_commit(merge_commit_oid)?;

    // Get parent OIDs
    let parent_oids: Vec<Oid> = merge_commit.parents().map(|p| p.id()).collect();

    // Assume the first parent is the main branch (adjust if necessary)
    //let main_parent_oid = parent_oids[0];

    // Find the base commit (LCA)
    let base_oid = repo.merge_base_many(&parent_oids)?;

    // Prepare a set to collect commits
    //let mut merge_commits = Vec::new();

    // Collect commits from each merged branch (excluding main parent)
    let mut last_commit_oid = None;
    for &parent_oid in &parent_oids[1..] {
        let mut revwalk = repo.revwalk()?;
        revwalk.push(parent_oid)?;
        revwalk.hide(base_oid)?;

        // Optional: Set sorting mode (e.g., topological order)
        revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;

        for commit_id_result in revwalk {
            let commit_id = commit_id_result?;
            let commit = repo.find_commit(commit_id)?;
            //merge_commits.push(commit);
            last_commit_oid = Some(commit_id);
            println!("  - {}: {}", commit.id(), commit.summary().unwrap());
        }
    }

    //Ok(merge_commits)
    log::info!("print_merge_commits:-");
    Ok(last_commit_oid)
}

// This must be improved currently it only handles simple merges
// and also need to decide what to do when root commits are encounterd
// expecially if there are multiple roots.
fn _pr_commits(repo_path: &str, merge_oid: &Oid) -> Result<Option<Oid>, Error> {
    let repo = Repository::open(repo_path)?;
    let merge_commit = repo.find_commit(*merge_oid)?;

    // Ensure this is a merge commit
    assert!(merge_commit.parent_count() > 1);

    // Parent 1: Base branch
    let base_oid = merge_commit.parent_id(0)?;
    // Parent 2: Tip of the PR branch
    let pr_tip_oid = merge_commit.parent_id(1)?;

    let mut prev_pr_commit: Option<Commit> = None;
    // Traverse the PR branch from its tip back to the base
    let mut pr_commit = repo.find_commit(pr_tip_oid)?;
    println!("Commits in the PR branch:");

    // This doesn't work expect for simple merges where base_oid is the
    // first parent of the merge commit and the last commit in the PR branch
    // points to the base_oid. Also, this should be filtered by the subdir.
    while pr_commit.id() != base_oid {
        println!(
            "  - {}: {}",
            pr_commit.id(),
            pr_commit.summary().unwrap_or("No summary")
        );
        if pr_commit.parent_count() == 0 {
            // Reached a root commit
            return Ok(None);
        }
        prev_pr_commit = Some(pr_commit.clone());
        pr_commit = pr_commit.parent(0)?;
    }

    if let Some(prev_pr_commit) = prev_pr_commit {
        Ok(Some(prev_pr_commit.id()))
    } else {
        Err(Error::new(
            git2::ErrorCode::NotFound,
            git2::ErrorClass::None,
            "No commits in PR branch",
        ))
    }
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
