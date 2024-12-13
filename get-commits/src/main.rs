use git2::{Commit, DiffFormat, DiffOptions, Oid, Repository, Tree};
use std::{env, error::Error};

fn collect_diff_files(
    repo: &Repository,
    from_tree: Option<&Tree>,
    to_tree: Option<&Tree>,
    modified_files: &mut Vec<String>,
    non_matching_count: &mut usize,
) -> Result<(), Box<dyn Error>> {
    let diff = repo.diff_tree_to_tree(from_tree, to_tree, None)?;
    for delta in diff.deltas() {
        if let Some(path) = delta.new_file().path() {
            let path_str = path.to_string_lossy();
            if path_str.starts_with("") {
                // Adjust for subdir filtering
                modified_files.push(path_str.to_string());
            } else {
                *non_matching_count += 1;
            }
        }
    }
    Ok(())
}

fn create_merged_baseline_tree<'repo>(
    repo: &'repo Repository,
    parents: &[Commit<'repo>],
) -> Result<Tree<'repo>, Box<dyn Error>> {
    if parents.len() < 2 {
        return Err("At least two parents are required for a merge".into());
    }

    // Get our tree and their tree
    let our_tree = parents[0].tree()?;
    let their_tree = parents[1].tree()?;

    // Find the common ancestor (base tree)
    let ancestor_tree = repo
        .merge_base(parents[0].id(), parents[1].id())
        .ok()
        .and_then(|oid| repo.find_commit(oid).ok())
        .map(|commit| commit.tree())
        .transpose()?;

    // Perform the merge
    let mut merge_index = repo.merge_trees(
        ancestor_tree.as_ref().ok_or("No common ancestor found")?,
        &our_tree,
        &their_tree,
        None,
    )?;

    // Write the result of the merge to a tree
    let merged_tree = repo.find_tree(merge_index.write_tree_to(repo)?)?;

    Ok(merged_tree)
}

fn log_tree_diff(
    repo: &Repository,
    from_tree: &Tree,
    to_tree: &Tree,
    label: &str,
) -> Result<(), Box<dyn Error>> {
    println!("Diff: {label}");

    // Create and configure DiffOptions
    let mut opts = DiffOptions::new();
    opts.include_untracked(true)
        .include_ignored(false)
        .context_lines(3)
        .recurse_untracked_dirs(true) // To include untracked directories
        .include_unmodified(false);

    let diff = repo.diff_tree_to_tree(Some(from_tree), Some(to_tree), Some(&mut opts))?;

    // Print the diff in Patch format with prefixes
    diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
        // Prefix for added/removed/unchanged lines
        let prefix = match line.origin() {
            '+' => "+", // Line added
            '-' => "-", // Line removed
            ' ' => " ", // Unchanged line
            '@' => "@", // Hunk header
            _ => " ",   // Fallback for other types
        };

        // Print the prefixed line
        print!(
            "{}{}",
            prefix,
            std::str::from_utf8(line.content()).unwrap_or("[INVALID UTF-8]")
        );

        true
    })?;

    println!("Diff: {label} END");
    Ok(())
}

fn process_commit(repo: &Repository, commit: &Commit) -> Result<(), Box<dyn Error>> {
    let commit_tree = commit.tree()?;
    let parents = commit.parents().collect::<Vec<_>>();

    let mut modified_files = Vec::new();
    let mut non_matching_count = 0;

    if parents.len() > 1 {
        // Handle merge commits
        let merged_baseline_tree = create_merged_baseline_tree(repo, &parents)?;
        let _ = log_tree_diff(
            repo,
            &merged_baseline_tree,
            &commit_tree,
            "merged_baseline_tree vs commit_tree",
        );
        collect_diff_files(
            repo,
            Some(&merged_baseline_tree),
            Some(&commit_tree),
            &mut modified_files,
            &mut non_matching_count,
        )?;
    } else if let Some(parent) = parents.first() {
        // Handle non-merge commits
        let parent_tree = parent.tree()?;
        let _ = log_tree_diff(repo, &parent_tree, &commit_tree, "parent vs commit_tree");
        collect_diff_files(
            repo,
            Some(&parent_tree),
            Some(&commit_tree),
            &mut modified_files,
            &mut non_matching_count,
        )?;
    }

    println!(
        "commit id: {}, summary: '{}', parent.len: {}, modified_count: {}, non_matching_count: {}, parents: {:?}",
        commit.id(),
        commit.summary().unwrap_or(""),
        parents.len(),
        modified_files.len(),
        non_matching_count,
        parents.iter().map(|p| p.id()).collect::<Vec<_>>(),
    );

    for file in &modified_files {
        println!("  file: {}", file);
    }

    Ok(())
}

fn get_commits(repo_path: &str, oid_strings: &[String]) -> Result<(), Box<dyn Error>> {
    log::info!("get_commits:+ repo_path: {repo_path}, oid_strings: {oid_strings:?}");
    let repo = Repository::open(repo_path)?;

    let oids = oid_strings
        .iter()
        .map(|oid_str| Oid::from_str(oid_str))
        .collect::<Result<Vec<_>, _>>()?;

    if oids.is_empty() || oids.len() > 2 {
        return Err(format!("We need one or two oids, but got {}", oids.len()).into());
    }

    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;
    revwalk.push(oids[0])?;
    if oids.len() == 2 {
        revwalk.hide(oids[1])?;
    }

    for oid in revwalk.filter_map(Result::ok) {
        let commit = repo.find_commit(oid)?;
        process_commit(&repo, &commit)?;
    }

    log::info!("get_commits:- repo_path: {repo_path}, oid_strings: {oid_strings:?}");
    Ok(())
}

fn usage() {
    eprintln!(
        "Usage: {} <repo_path> <from_oid> {{<to_oid>}}",
        env::args().next().unwrap()
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    custom_logger::env_logger_init("info");

    log::info!("main:+");

    let mut args_iter = env::args();
    args_iter.next(); // Skip executable name

    let repo_path = args_iter.next().ok_or("Repository path not provided")?;
    let oid_strings: Vec<_> = args_iter.collect();

    if oid_strings.is_empty() {
        usage();
        return Err("OIDs not provided".into());
    }

    let result = get_commits(&repo_path, &oid_strings);

    log::info!("main:-");
    result
}
