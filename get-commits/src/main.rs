use std::{env, error::Error};

use git2::Repository;

fn get_commits_between(repo_path: &str, oid_strings: &Vec<String>) -> Result<(), Box<dyn Error>> {
    log::info!("get_commits_between:+ repo_path: {repo_path}, oid_strings: {oid_strings:?}");
    let result = Repository::open(repo_path);
    log::info!(
        "get_commits_between: Repository::open result: is_ok={}",
        result.is_ok()
    );
    let repo = result?;

    let mut oids = Vec::new();
    for oid_str in oid_strings {
        let oid = git2::Oid::from_str(oid_str)?;
        oids.push(oid);
    }

    if oids.is_empty() || oids.len() > 2 {
        return Err(format!("We need one or two oids, there were {}", oids.len()).into());
    }

    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;

    // Push the starting commit and hide the ending commit if present
    revwalk.push(oids[0])?;
    if oids.len() == 2 {
        revwalk.hide(oids[1])?;
    }

    // Collect the commits
    let commits = revwalk.filter_map(Result::ok).collect::<Vec<_>>();
    for commit in commits {
        let commit = repo.find_commit(commit)?;
        let parents = commit.parents();
        println!(
            "commit id: {}, summary: '{}', parent.len: {}, parents: {:?}",
            commit.id(),
            commit.summary().unwrap_or(""),
            commit.parents().len(),
            parents.collect::<Vec<_>>(),
        );
    }

    log::info!("get_commits_between:- repo_path: {repo_path}, oid_strings: {oid_strings:?}");
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

    _ = args_iter.next(); // Skip executable name the first parameter

    // Get the repository path which is the first argument
    let repo_path = match args_iter.next() {
        Some(path) => path,
        None => {
            usage();
            return Err("Repository path not provided".into());
        }
    };

    // Get the OIDs
    let mut oid_strings = Vec::new();
    for arg in args_iter {
        log::info!("arg: {arg}");
        oid_strings.push(arg);
    }

    // Get the commits between the OIDs
    let result = get_commits_between(&repo_path, &oid_strings);

    log::info!("main:-");
    result
}
