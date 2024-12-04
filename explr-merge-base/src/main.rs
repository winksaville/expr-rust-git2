use std::env;

use git2::{Error, Repository};

fn explr_merge_base(repo_path: &str, oid1_str: &str, oid2_str: &str) -> Result<(), Error> {
    log::info!(
        "explr_merge_base:+ repo_path: {repo_path}, oid1_str: {oid1_str}, oid2_str: {oid2_str}"
    );
    let repo = Repository::open(repo_path)?;

    let oid1 = git2::Oid::from_str(oid1_str)?;
    let oid2 = git2::Oid::from_str(oid2_str)?;

    let oid = repo.merge_base(oid1, oid2)?;
    println!("{}", oid);

    log::info!(
        "explr_merge_base:1 repo_path: {repo_path}, oid1_str: {oid1_str}, oid2_str: {oid2_str}"
    );
    Ok(())
}

fn usage() {
    eprintln!(
        "Usage: {} <repo_path> <oid1> <oid2>",
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
    let oid1_string = match env::args().nth(2) {
        Some(oid) => oid,
        None => {
            usage();
            return Err("oid1 not provided".into());
        }
    };
    let oid2_string = match env::args().nth(3) {
        Some(oid) => oid,
        None => {
            usage();
            return Err("oid2 not provided".into());
        }
    };
    if let Err(e) = explr_merge_base(&repo_path, &oid1_string, &oid2_string) {
        eprintln!("Error: {e}");
        return Err(e.into());
    }

    log::info!("main:-");
    Ok(())
}
