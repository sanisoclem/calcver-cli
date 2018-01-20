#[macro_use]
extern crate clap;
extern crate calcver;
extern crate git2;

use clap::{App, Arg, SubCommand};
use calcver::config::{ProjectConfig};
use calcver::{VersionBumpBehavior};

mod repogit;

fn main() {
    let matches = App::new("calcver")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Calculate your project's next version")
        .arg(Arg::with_name("repo")
                    .help("Path to the repository")
                    .index(0))
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

    let configPath = match matches.value_of("config") {
        Some(path)=> path,
        _=> "calcver.yml"
    };

    // -- TODO: override with values from config file (if present)
    let config = ProjectConfig::from_defaults();
    let is_release = matches.is_present("release");

    let version = calcver::get_version(&config,&repo,VersionBumpBehavior::Auto,is_release).unwrap();

    if is_release { // && !dryrun {
        println!("Performing release actions...");
        // TODO: bump, commit, tag
    }

    println!("Next version is: {}",version);
    


    // if let Some(c) = matches.value_of("config") {
    //     println!("Value for config: {}", c);
    // }

    // // You can see how many times a particular flag or argument occurred
    // // Note, only flags can have multiple occurrences
    // match matches.occurrences_of("debug") {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is kind of on"),
    //     2 => println!("Debug mode is on"),
    //     3 | _ => println!("Don't be crazy"),
    //  }

    // // You can check for the existence of subcommands, and if found use their
    // // matches just as you would the top level app
    //  if let Some(ref matches) = matches.subcommand_matches("test") {
    //      // "$ myapp test" was run
    //      if matches.is_present("list") {
    //          // "$ myapp test -l" was run
    //          println!("Printing testing lists...");
    //      } else {
    //          println!("Not printing testing lists...");
    //      }
    //  }
}