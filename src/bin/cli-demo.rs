use git2::*;
use std::env;
use std::path::PathBuf;
use std::process::Command;

const BITCOIN_GENESIS: &str = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f";

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

fn git_init() -> core::result::Result<(), git2::Error> {
    #[allow(clippy::if_same_then_else)]
    let mut command = Command::new("printenv");

    let git_author_date = "GIT_AUTHOR_DATE";
    //println!("{}", git_author_date);
    let git_author_date_value = "Thu, 01 Jan 1970 00:00:00 +0000";
    //println!("{}", git_author_date_value);
    let git_committer_date = "GIT_COMMITTER_DATE";
    //println!("{}", git_committer_date);
    let git_committer_date_value = "Thu, 01 Jan 1970 00:00:00 +0000";
    //println!("{}", git_committer_date_value);
    let git_command = "GIT_COMMAND";
    //println!("{}", git_command);
    let git_command_string = "commit --allow-empty -m 'Initial commit'";
    //println!("{}", git_command_string);

    command.env(git_author_date, git_author_date_value);
    command.env(git_committer_date, git_committer_date_value);
    command.env(git_command, git_command_string);

    let printenv = command.output().expect("failed to execute process");
    let mut _result = String::from_utf8(printenv.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    println!("{}", _result);

    let mut git_init = Command::new("git");
    git_init
        .args(["init"])
        .output()
        .expect("failed to execute process");

    let git_init = command.output().expect("failed to execute process");
    let mut _result = String::from_utf8(git_init.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    println!("git_init:{}", _result);

    let mut git_empty_commit = Command::new("git");
    git_empty_commit
        .args(["commit", "--allow-empty", "-m", "init", "--no-gpg-sign"])
        .output()
        .expect("failed to execute process");

    let git_empty_commit = command.output().expect("failed to execute process");
    let mut _result = String::from_utf8(git_empty_commit.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    println!("git_init:{}", _result);

    // let initialize = if cfg!(target_os = "windows") {
    //     command
    //         .args(["/C", "commit --allow-empty -m init commit", ""])
    //         .output()
    //         .expect("failed to execute process")
    // //} else if cfg!(target_os = "macos") {
    // //    command
    // //        .args([
    // //            "commit --allow-empty -m init commit",
    // //            "",
    // //        ])
    // //        .output()
    // //        .expect("failed to execute process")
    // //} else if cfg!(target_os = "linux") {
    // //    command
    // //        .args([
    // //            "commit --allow-empty -m init commit",
    // //            "",
    // //        ])
    // //        .output()
    // //        .expect("failed to execute process")
    // } else {
    //     let mut command = Command::new("sh");
    //     command
    //         .args(["git commit --allow-empty -m init commit", ""])
    //         //.arg("git -h &&")
    //         //  //.args([
    //         //  //    "git",
    //         //  //    "commit",
    //         //  //    "--allow-empty",
    //         //  //    "-m",
    //         //  //    "init commit",
    //         //  //    "",
    //         //  //])
    //         .output()
    //         .expect("failed to execute process")
    // };

    // let result = String::from_utf8(initialize.stdout)
    //     .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
    //     .unwrap();
    // println!("___________{}", result);

    //let repo_path = ".";
    //let _repo = Repository::init(repo_path)?;
    //let _repo = match Repository::open(".") {
    //    Ok(_repo) => _repo,
    //    Err(e) => panic!("Error opening repository: {}", e),
    //};
    Ok(())
}
fn main() -> core::result::Result<(), git2::Error> {
    let _init_result = git_init();
    //println!("_init_result={:?}", _init_result);

    //let repo_path = ".";
    //let repo = Repository::init(repo_path)?;
    //let repo = match Repository::open(".") {
    //    Ok(repo) => repo,
    //    Err(e) => panic!("Error opening repository: {}", e),
    //};
    //let status = repo.statuses()?;
    Ok(())
}
