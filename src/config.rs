use libcalcver;

pub struct RepositoryConfig {
    scm_type: String,
    path: String
}

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
    pub repository: RepositoryConfig,
    pub release: Vec<ReleaseConfig>
}


pub fn from_config(config: &str) -> CalcverConfig {
    CalcverConfig {
        project: libcalcver::config::ProjectConfig::from_defaults(),
        repository: RepositoryConfig {
            scm_type: "git".to_string(),
            path: ".".to_string()
        },
        release: vec![ReleaseConfig::CargoToml("cargo.toml".to_string())]
    }
}
