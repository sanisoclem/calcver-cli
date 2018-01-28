#[macro_use]
extern crate clap;
extern crate libcalcver;
extern crate git2;

use clap::{App, Arg};
use libcalcver::{VersionBumpBehavior};

mod repogit;
mod config;

static DEFAULT_CONFIG_NAME: &'static str = ".calcver.yml";

fn main() {
    let matches = App::new("calcver")
        .version(crate_version!())
        .about("Calculate your project's next version")
        .arg(Arg::with_name("repo")
                    .help("Path to the repository")
                    .default_value(".")
                    .index(1))
        .arg(Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .default_value(DEFAULT_CONFIG_NAME)
                    .value_name("FILE")
                    .help("Use a config file")
                    .takes_value(true))
        .arg(Arg::with_name("release")
                    .short("R")
                    .long("release")
                    .help("Perform a tagged release"))
        .arg(Arg::with_name("dryrun")
                    .short("n")
                    .long("dryrun")
                    .help("Dry run"))
        
        .get_matches();

    // -- get variables
    let config_path = matches.value_of("config").unwrap_or(DEFAULT_CONFIG_NAME);
    let repo_path = matches.value_of("repo").unwrap_or(".");
    let is_release = matches.is_present("release");
    let is_dryrun = matches.is_present("dryrun");

    let version = run(config_path,repo_path,is_release,is_dryrun);

    println!("Next version is: {}",version);
}

fn run(config_path: &str, repo_path: &str, release: bool, dry_run: bool) -> String {
    // -- parse config if existing
    let config = config::from_config(config_path);

    // -- get git repo
    let repo = repogit::GitRepo::from(repo_path);

    // -- get the next version
    let version = libcalcver::get_version(&config.project,&repo,VersionBumpBehavior::Auto,release).unwrap();

    if release { // && !dryrun {
        
    }

    version
}
