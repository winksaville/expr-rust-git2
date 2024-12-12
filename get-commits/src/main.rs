use std::{env, error::Error};
use git2::{Commit, Oid, Repository, Tree};

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
            if path_str.starts_with("") { // Adjust for subdir filtering
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

    // Assume the first parent is "ours" and the second is "theirs"
    let our_tree = parents[0].tree()?;
    let their_tree = parents[1].tree()?;

    // Attempt to find a common ancestor (base tree)
    let ancestor = repo.merge_base(parents[0].id(), parents[1].id())
        .ok()
        .and_then(|oid| repo.find_commit(oid).ok())
        .map(|commit| commit.tree())
        .transpose()?;

    // Perform the merge
    let mut merge_index = repo.merge_trees(
        ancestor.as_ref().ok_or("No common ancestor found")?,
        &our_tree,
        &their_tree,
        None,
    )?;

    // Write the result of the merge to a tree
    Ok(repo.find_tree(merge_index.write_tree_to(repo)?)?)
}


fn process_commit(repo: &Repository, commit: &Commit) -> Result<(), Box<dyn Error>> {
    let commit_tree = commit.tree()?;
    let parents = commit.parents().collect::<Vec<_>>();

    let mut modified_files = Vec::new();
    let mut non_matching_count = 0;

    if parents.len() > 1 {
        // Handle merge commits
        let merged_baseline_tree = create_merged_baseline_tree(repo, &parents)?;
        collect_diff_files(repo, Some(&merged_baseline_tree), Some(&commit_tree), &mut modified_files, &mut non_matching_count)?;
    } else if let Some(parent) = parents.first() {
        // Handle non-merge commits
        let parent_tree = parent.tree()?;
        collect_diff_files(repo, Some(&parent_tree), Some(&commit_tree), &mut modified_files, &mut non_matching_count)?;
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

    if oids.len() < 1 || oids.len() > 2 {
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

    let repo_path = args_iter.next().ok_or_else(|| "Repository path not provided")?;
    let oid_strings: Vec<_> = args_iter.collect();

    if oid_strings.is_empty() {
        usage();
        return Err("OIDs not provided".into());
    }

    let result = get_commits(&repo_path, &oid_strings);

    log::info!("main:-");
    result
}
