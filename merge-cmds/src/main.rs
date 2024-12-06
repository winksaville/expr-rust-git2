use std::env;

use git2::{Error, Repository};

fn explr_merge(repo_path: &str, oid_strings: &Vec<String>) -> Result<(), Error> {
    log::info!("explr_merge:+ repo_path: {repo_path}, oid_strings: {oid_strings:?}");
    let result = Repository::open(repo_path);
    log::info!("explr_merge: result: is_ok={}", result.is_ok());
    let repo = result?;

    let mut oids = Vec::new();
    for oid_str in oid_strings {
        let oid = git2::Oid::from_str(oid_str)?;
        oids.push(oid);
    }

    // Should be at least 2 oids
    if oid_strings.len() < 2 {
        return Err(Error::from_str("At least two OIDs are required"));
    }

    let oid1 = git2::Oid::from_str(&oid_strings[0])?;
    let oid2 = git2::Oid::from_str(&oid_strings[1])?;
    log::info!("explr_merge: call merge_base oid1: {oid1}, oid2: {oid2}");
    let result = repo.merge_base(oid1, oid2)?;
    println!("merge_base result: {result:?}");

    let result = repo.merge_base_many(&oids)?;
    println!("merge_base_many result: {result:?}");

    let result = repo.merge_bases(oid1, oid2)?;
    println!("merge_bases result: {result:?}");

    let result = repo.merge_base_octopus(&oids)?;
    println!("merge_base_octopus result: {result:?}");

    log::info!("explr_merge:- repo_path: {repo_path}, oid_strings: {oid_strings:?}");
    Ok(())
}

fn usage() {
    eprintln!(
        "Usage: {} <repo_path> <oid1> <oid2> [ oid3 ..]",
        env::args().next().unwrap()
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    custom_logger::env_logger_init("none");

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

    // Call the explr_merge function
    if let Err(e) = explr_merge(&repo_path, &oid_strings) {
        eprintln!("Error: {e}");
        return Err(e.into());
    }

    log::info!("main:-");
    Ok(())
}
