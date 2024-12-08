use std::{env, error::Error};

use git2::Repository;

fn merge_cmds(repo_path: &str, oid_strings: &Vec<String>) -> Result<(), Box<dyn Error>> {
    log::info!("merge_cmds:+ repo_path: {repo_path}, oid_strings: {oid_strings:?}");
    let result = Repository::open(repo_path);
    log::info!(
        "merge_cmds Repository::open result: is_ok={}",
        result.is_ok()
    );
    let repo = result?;

    let mut oids = Vec::new();
    for oid_str in oid_strings {
        let oid = git2::Oid::from_str(oid_str)?;
        oids.push(oid);
    }

    if oids.is_empty() {
        return Err("We need at least one oid".into());
    }

    // If only one oid is provided the second oid is set to zero
    let oid1 = if let Ok(oid) = git2::Oid::from_str(&oid_strings[0]) {
        oid
    } else {
        return Err(format!("The first 'oid' isn't valid: '{}'", oid_strings[0]).into());
    };

    // Get the second oid, if it is not provided it is set to zero

    //// Poor version as there two calls to git2::Oid::zero()
    //let oid2 = match oid_strings.get(1) {
    //    Some(oid_str) => match git2::Oid::from_str(oid_str) {
    //        Ok(oid) => oid,
    //        Err(_) => git2::Oid::zero(),
    //    },
    //    None => git2::Oid::zero(),
    //};

    // "Eager" evaluation, the git2::Oid::zero() is called even if the first oid is invalid.
    // I don't like this version as to me it's not clear that the git2::Oid::zero()
    // is always called, but it would be unless the compiler can prove there are no side
    // effects.
    //let oid2 = oid_strings
    //    .get(1)
    //    .and_then(|oid_str| git2::Oid::from_str(oid_str).ok())
    //    .unwrap_or(git2::Oid::zero());

    // "Lazy" evaluation, the git2::Oid::zero() is called only if the first oid is invalid
    let oid2 = oid_strings
        .get(1)
        .and_then(|oid_str| git2::Oid::from_str(oid_str).ok())
        .unwrap_or_else(|| git2::Oid::zero());

    log::info!("merge_cmds call merge_base oid1: {oid1}, oid2: {oid2}");
    let result = repo.merge_base(oid1, oid2);
    log::info!("merge_cmds merge_base result: {result:?}");

    log::info!("merge_cmds call merge_base_many oids: {oids:?}");
    let result = repo.merge_base_many(&oids);
    log::info!("merge_cmds merge_base_many result: {result:?}");

    log::info!("merge_cmds call merge_bases oid1: {oid1} oid2: {oid2}");
    let result = repo.merge_bases(oid1, oid2);
    log::info!("merge_cmds merge_bases result: {result:?}");

    log::info!("merge_cmds call merge_base_otopus oids: {oids:?}");
    let result = repo.merge_base_octopus(&oids);
    log::info!("merge_cmds merge_base_octopus result: {result:?}");

    log::info!("merge_cmds call merge_bases_many oids: {oids:?}");
    let result = repo.merge_bases_many(&oids);
    log::info!("merge_cmds merge_bases_many result: {result:?}");

    log::info!("merge_cmds:-");
    Ok(())
}

fn usage() {
    eprintln!(
        "Usage: {} <repo_path> <oid1> <oid2> [ oid3 ..]",
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

    // Get the OIDs which are the remaining arguments
    let mut oid_strings = Vec::new();
    //while let Some(arg) = args_iter.next() {
    for arg in args_iter {
        log::info!("arg: {arg}");
        oid_strings.push(arg);
    }

    // Call the merge_cmds function
    let result = merge_cmds(&repo_path, &oid_strings);

    log::info!("main:-");
    result
}
