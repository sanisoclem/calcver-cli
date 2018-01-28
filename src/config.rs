use libcalcver;
use repo;

pub enum ReleaseConfig {
    FileReplace {
        files: String, //glob
        replace_match: String, // regex to look for 
    },
    CargoToml(String),
    Csproj(String)
}

pub struct CalcverConfig {
    pub project: libcalcver::config::ProjectConfig,
    pub repository: repo::RepositoryConfig,
    pub release: Vec<ReleaseConfig>
}

pub fn from_config(_config: &str) -> CalcverConfig {
    CalcverConfig {
        project: libcalcver::config::ProjectConfig::from_defaults(),
        repository: repo::RepositoryConfig {
            scm_type: "git".to_string(),
            path: ".".to_string()
        },
        release: vec![ReleaseConfig::CargoToml("cargo.toml".to_string())]
    }
}
