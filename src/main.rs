#[macro_use]
extern crate clap;
extern crate libcalcver;
extern crate git2;
extern crate toml;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;

use clap::{App, Arg};
use libcalcver::{VersionBumpBehavior};

mod repo;
mod repogit;
mod config;
mod release;

use repo::{CodeRepository};

static DEFAULT_CONFIG_NAME: &'static str = ".calcver.yml";

fn main() {
    let matches = App::new("calcver")
        .version(crate_version!())
        .about("Calculate your project's next version")
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
    let is_release = matches.is_present("release");
    let is_dryrun = matches.is_present("dryrun");

    let version = run(config_path,is_release,is_dryrun);

    println!("Next version is: {}",version);
}

pub fn run(config_path: &str, release: bool, _dry_run: bool) -> String {
    // -- parse config if existing
    let config = config::from_config(config_path);

    // -- get repo
    // -- todo: find some rust way to move this to repo module
    let repo  = match config.repository.scm_type.as_ref() { 
        "git" => Ok(config.repository.get_repo::<repogit::GitRepo>()),
        _ => Err("not supported")
    }.unwrap();

    // -- get the next version
    let version = libcalcver::get_version(&config.project,&repo,VersionBumpBehavior::Auto,release).unwrap();

    // -- execute any actions defined
    for action in config.release.iter() {
        action.run(&config.repository.path,&version);
    }

    // -- if releasing, commit and tag
    if release { // && !dryrun {
        repo.commit(&version);
    }

    version
}
