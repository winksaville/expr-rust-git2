use git2::{Repository, Tree, Error};

fn commits_for_subdir(repo_path: &str, subdir: &str) -> Result<(), Error> {
    let repo = Repository::open(repo_path)?;
    let mut revwalk = repo.revwalk()?;

    // Push HEAD to start traversal
    revwalk.push_head()?;

    // Previous commit's tree
    let mut prev_tree: Option<Tree> = None;

    for oid_result in revwalk {
        let oid = oid_result?;
        let commit = repo.find_commit(oid)?;

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
                            found = true;
                        }
                    }
                    if let Some(path) = delta.new_file().path() {
                        if path.starts_with(subdir) {
                            found = true;
                        }
                    }
                    if found {
                        // Stop further diff processing if we find a match
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
    }

    Ok(())
}

fn main() {
    if let Err(e) = commits_for_subdir("/path/to/repo", "subdir/path") {
        eprintln!("Error: {}", e);
    }
}
