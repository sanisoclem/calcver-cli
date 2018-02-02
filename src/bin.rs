#[macro_use]
extern crate clap;
extern crate git2;
extern crate toml;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;
#[macro_use(quick_error)]
extern crate quick_error;
extern crate regex;
extern crate semver;


use clap::{App, Arg};

mod repository;
mod repogit;
mod config_file;
mod config;
mod release;
mod version;
mod error;

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

    let version = version::run(config_path,is_release,is_dryrun);

    println!("Next version is: {}",version);
}