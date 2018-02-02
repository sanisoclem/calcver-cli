pub static TAG_REGEX_DEFAULT: &'static str = r"\d+\.\d+\.\d+";
pub static MAJOR_REGEX_DEFAULT: &'static str = "BREAKING CHANGE:";
pub static MINOR_REGEX_DEFAULT: &'static str = "^feat";
pub static PATCH_REGEX_DEFAULT: &'static str = "^fix";
pub static PRERELEASE_PREFIX_DEFAULT: &'static str = "alpha";

use repository;
use release;


pub struct CalcverConfig {
    pub project: VersionConfig,
    pub repository: repository::RepositoryConfig,
    pub release: Vec<release::ReleaseConfig>
}

pub struct VersionConfig {
    pub prerelease_prefix: String,
    pub tag_regex: String,
    pub major_regex: String,
    pub minor_regex: String,
    pub patch_regex: String
}

impl VersionConfig {
    pub fn from_defaults () -> VersionConfig {
        VersionConfig {
            prerelease_prefix: String::from(PRERELEASE_PREFIX_DEFAULT),
            tag_regex: String::from(TAG_REGEX_DEFAULT),
            major_regex: String::from(MAJOR_REGEX_DEFAULT),
            minor_regex: String::from(MINOR_REGEX_DEFAULT),
            patch_regex: String::from(PATCH_REGEX_DEFAULT),
        }
    }
}