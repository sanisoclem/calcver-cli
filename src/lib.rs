extern crate git2;
extern crate toml;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;
#[macro_use(quick_error)]
extern crate quick_error;
extern crate regex;
extern crate semver;

pub mod config;
pub mod error;
pub mod repository;
pub mod release;
pub mod config_file;
pub mod repogit;
mod version;

pub use version::*;

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyRepo {
        pub  commits: Vec<String>,
        pub last_tag : Option<String>
    }
    impl repository::CodeRepository for DummyRepo {
        fn get_last_tag(&self) -> Option<&str> {
            match self.last_tag {
                Some(ref tag) => Some(tag),
                None=> None
            }
        }
        fn get_commits_since_last_tag(&self) -> &Vec<String> {
            &self.commits
        }
        fn commit(&self, _tag: &str) {

        }
    }

    #[test]
    fn smoke_test(){
        let repo = DummyRepo { 
            commits: vec!["feat: smoke test".to_string()],
            last_tag: Some("v1.2.3".to_string())
        };
        let config =  config::VersionConfig::from_defaults();

        assert_eq!("1.3.0-alpha.1", get_version(&config,&repo,VersionBumpBehavior::Auto,false).unwrap());
        assert_eq!("1.3.0", get_version(&config,&repo,VersionBumpBehavior::Auto,true).unwrap())
    }

    #[test]
    fn smoke_test_release(){
        let repo = DummyRepo { 
            commits: vec![],
            last_tag: Some("v1.2.3".to_string())
        };
        let config =  config::VersionConfig::from_defaults();
        assert_eq!("1.2.3", get_version(&config,&repo,VersionBumpBehavior::Auto,true).unwrap())
    }

    #[test]
    fn error_if_no_info(){
        let repo = DummyRepo { 
            commits: vec![],
            last_tag: None
        };
        let config =  config::VersionConfig::from_defaults();
        assert!(get_version(&config,&repo,VersionBumpBehavior::Auto,true).is_err())
    }

    #[test]
    fn error_if_invalid_regex(){
        let repo = DummyRepo { 
            commits: vec!["feat: smoke test".to_string()],
            last_tag: Some("v1.2.3".to_string())
        };
        let config =  config::VersionConfig {
            prerelease_prefix: String::from(config::PRERELEASE_PREFIX_DEFAULT),
            tag_regex: String::from(config::TAG_REGEX_DEFAULT),
            major_regex: String::from(config::MAJOR_REGEX_DEFAULT),
            minor_regex: String::from(config::MINOR_REGEX_DEFAULT),
            patch_regex: String::from("invalidregex[\\t"),
        };
        assert!(get_version(&config,&repo,VersionBumpBehavior::Auto,true).is_err())
    }
}
