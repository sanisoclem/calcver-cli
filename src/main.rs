#[macro_use]
extern crate clap;
extern crate libcalcver;
extern crate git2;

use clap::{App, Arg};
use libcalcver::config::{ProjectConfig};
use libcalcver::{VersionBumpBehavior};

mod repogit;

fn main() {
    let matches = App::new("calcver")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Calculate your project's next version")
        .arg(Arg::with_name("repo")
                    .help("Path to the repository")
                    .default_value(".")
                    .index(1))
        .arg(Arg::with_name("config")
                    .short("c")
                    .long("config")
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

    let repo = match matches.value_of("repo") {
        Some(path) => repogit::GitRepo::from(path),
        _ => repogit::GitRepo::from(".")
    };

    let config_path = match matches.value_of("config") {
        Some(path)=> path,
        _=> "calcver.yml"
    };

    // -- TODO: override with values from config file (if present)
    let config = ProjectConfig::from_defaults();
    let is_release = matches.is_present("release");

    let version = libcalcver::get_version(&config,&repo,VersionBumpBehavior::Auto,is_release).unwrap();

    if is_release { // && !dryrun {
        println!("Performing release actions...");
        // TODO: bump, commit, tag
    }

    println!("Next version is: {}",version);
}