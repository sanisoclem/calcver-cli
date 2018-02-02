use libcalcver;
use serde_yaml;
use repo;
use release;
use std::fs::{File};
use std::io::prelude::*;

pub struct CalcverConfig {
    pub project: libcalcver::config::ProjectConfig,
    pub repository: repo::RepositoryConfig,
    pub release: Vec<release::ReleaseConfig>
}

#[derive(Debug, Deserialize)]
pub struct CalcverFileConfig {
    pub repository_type: Option<String>,
    pub root: Option<String>,
    pub prerelease_prefix: Option<String>,
    pub tag_regex:  Option<String>,
    pub major_regex: Option<String>,
    pub minor_regex: Option<String>,
    pub patch_regex: Option<String>,
    pub cargo:  Option<String>,
    pub actions: Option<Vec<String>>
}

impl CalcverFileConfig {
     pub fn convert(self) -> CalcverConfig {
        let project_defaults = libcalcver::config::ProjectConfig::from_defaults();
        let mut actions: Vec<release::ReleaseConfig> = vec![];
        if let Some(a) = self.actions {
            for item in a {
                actions.push(release::ReleaseConfig::RunScript(item))
            }
        }
        if let Some(c) =  self.cargo {
            actions.push(release::ReleaseConfig::CargoToml(c));
        }
        CalcverConfig {
            project: libcalcver::config::ProjectConfig {
                commit_template: project_defaults.commit_template, // -- not needed anymore
                prerelease_prefix: self.prerelease_prefix.unwrap_or(project_defaults.prerelease_prefix),
                tag_regex: self.tag_regex.unwrap_or(project_defaults.tag_regex),
                major_regex: self.major_regex.unwrap_or(project_defaults.major_regex),
                minor_regex: self.minor_regex.unwrap_or(project_defaults.minor_regex),
                patch_regex: self.patch_regex.unwrap_or(project_defaults.patch_regex),
            },
            repository: repo::RepositoryConfig {
                scm_type: self.repository_type.unwrap_or("git".to_owned()),
                path: self.root.unwrap_or(".".to_owned())
            },
            release: actions
        }
    }
}


pub fn from_config(config_path: &str) -> CalcverConfig {
    let mut buffer = String::new();
    {
        let mut f = File::open(&config_path).unwrap();
        f.read_to_string(&mut buffer).unwrap();
    }
    parse_config(&buffer)
}

fn parse_config(config_yaml: &str) -> CalcverConfig {
    let retval: CalcverFileConfig = serde_yaml::from_str(&config_yaml).unwrap();
    retval.convert()
}



#[cfg(test)]
mod tests {
    use super::*;
    const NO_ACTIONS_CONFIG: &'static str = "repository_type: git";
    const MIN_CONFIG: &'static str = "cargo: .\\cargo.toml";
    const SCRIPTS_CONFIG: &'static str = r###"
actions:
  - .\bin\update-version.cmd
  - .\bin\publish.cmd
"###;
    const OVERRIDE_CONFIG: &'static str = r###"cargo: .\\cargo.toml
root: ..\src
repository_type: hg
"###;

    #[test]
    fn default_repo_is_git(){
        assert_eq!(parse_config(MIN_CONFIG).repository.scm_type,"git");
    }
    #[test]
    fn default_repo_path_is_current(){
        assert_eq!(parse_config(MIN_CONFIG).repository.path,".");
    }

    #[test]
    fn repository_type_is_configurable(){
        assert_eq!(parse_config(OVERRIDE_CONFIG).repository.scm_type,"hg");
    }
    #[test]
    fn repo_path_is_configurable(){
        assert_eq!(parse_config(OVERRIDE_CONFIG).repository.path,"..\\src");
    }

    #[test]
    fn cargo_toml_is_added_if_present(){
        let parsed = parse_config(MIN_CONFIG);
        let cargo = parsed.release.into_iter().find(|i| match i {
            &release::ReleaseConfig::CargoToml(ref t) => {
                assert_eq!(t,".\\cargo.toml");
                true
            },
        _=> false
        });
        
        assert!(cargo.is_some());
    }

    #[test]
    fn cargo_toml_is_not_added_if_not_defined(){
        let parsed = parse_config(SCRIPTS_CONFIG);
        let has_cargo = parsed.release.into_iter().any(|i| match i {
            release::ReleaseConfig::CargoToml(_) => {true},
            _=> false
        });
        
        assert!(!has_cargo);
    }

    #[test]
    fn no_actions_by_default(){
        let parsed = parse_config(NO_ACTIONS_CONFIG);
        assert!(parsed.release.len() == 0);
    }
}