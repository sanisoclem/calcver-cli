use libcalcver;
use repo;
use release;

pub struct CalcverConfig {
    pub project: libcalcver::config::ProjectConfig,
    pub repository: repo::RepositoryConfig,
    pub release: Vec<release::ReleaseConfig>
}

// -- hard coded for now
pub fn from_config(_config: &str) -> CalcverConfig {
    CalcverConfig {
        project: libcalcver::config::ProjectConfig::from_defaults(),
        repository: repo::RepositoryConfig {
            scm_type: "git".to_string(),
            path: ".".to_string()
        },
        release: vec![
            release::ReleaseConfig::CargoToml("cargo.toml".to_string()),
            release::ReleaseConfig::RunScript(".\\bin\\update-version.cmd".to_string()) 
        ]
    }
}
