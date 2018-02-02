use super::*;
use error;
use regex::{RegexSet,Regex};
use semver;

#[derive(PartialEq)]
pub enum VersionBumpBehavior {
    None,
    Auto,
    Major,
    Minor,
    Patch,
}

pub fn get_version(config:  &config::VersionConfig,repo: &repository::CodeRepository,  bump_behavior: VersionBumpBehavior, release: bool) -> Result<String,error::CalcverError> {
    let commits = repo.get_commits_since_last_tag();
    let last_tag = repo.get_last_tag();

    get_next_version(&config, bump_behavior, &commits, last_tag,release)
}

pub fn get_next_version(config: &config::VersionConfig, bump_behavior: VersionBumpBehavior, commits: &Vec<String>, last_tag: Option<&str>, release: bool) -> Result<String,error::CalcverError> {
    if commits.len() == 0 && release {
        return match last_tag {
            Some(tag) => Ok(get_current_version(&config,Some(tag))?),
            None => Err(error::CalcverError::of(error::CalcverErrorReason::NoCommitsOnRelease))
        }
    }
    let normalized_bump_behavior = match bump_behavior {
        VersionBumpBehavior::Auto => get_bump_behavior(&config, &commits)?,
        _=> bump_behavior
    };
    let current_version = get_current_version(&config,last_tag)?;
    bump_version(&config, normalized_bump_behavior, &current_version, release, commits.len())
}


fn bump_version(config: &config::VersionConfig, bump_behavior: VersionBumpBehavior, current_version: &str, release: bool,num_commits: usize) -> Result<String,error::CalcverError> {
    // if version bump behavior
    let v = semver::Version::parse(&current_version)?;
    
    let output = match bump_behavior {
        VersionBumpBehavior::Major=> semver::Version::new(v.major + 1, 0, 0),
        VersionBumpBehavior::Minor=> semver::Version::new(v.major, v.minor + 1, 0),
        VersionBumpBehavior::Patch=> semver::Version::new(v.major, v.minor , v.patch + 1),
        VersionBumpBehavior::None => semver::Version::new(v.major, v.minor , v.patch),
        _ => panic!("unexpected bump behavior")
    };
    match (release,bump_behavior) {
        (_,VersionBumpBehavior::None) => Ok(output.to_string()),
        (true,_) => Ok(output.to_string()),
        (false,_ ) => {
            let mut retval = output.to_string();
            retval.push_str("-");
            retval.push_str(&config.prerelease_prefix);
            retval.push_str(".");
            retval.push_str(num_commits.to_string().as_ref());
            Ok(retval)
        }
    }
}

fn get_bump_behavior(config: &config::VersionConfig, commit_messages: &Vec<String> ) -> Result<VersionBumpBehavior,error::CalcverError> {  
    let set = RegexSet::new(&[
        &config.major_regex,
        &config.minor_regex,
        &config.patch_regex,
    ])?; 
    let mut  bump_behavior = VersionBumpBehavior::None;

    for msg in commit_messages {
        let matches = set.matches(&msg);
        if matches.matched(0) {
            return Ok(VersionBumpBehavior::Major)
        } else if matches.matched(1) {
            bump_behavior = VersionBumpBehavior::Minor;
        } else if bump_behavior == VersionBumpBehavior::None {
            bump_behavior = VersionBumpBehavior::Patch;
        }
    }
    
    Ok(bump_behavior)
}

fn get_current_version(config: &config::VersionConfig,last_tag: Option<&str>) -> Result<String,error::CalcverError> {
    let r = Regex::new(&config.tag_regex)?;
    match last_tag {
        Some(tag) => match r.find(&tag) {
            Some(tag) => Ok(tag.as_str().to_string()),
            _ => Ok(String::from("0.0.0"))
        },
        _ => Ok(String::from("0.0.0"))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    fn get_config() -> config::VersionConfig {
        config::VersionConfig::from_defaults()
    }

    #[test]
    fn no_bump_if_no_commits(){
        let commits_since_last_tag = vec![];
        let config =  get_config();

        assert_eq!("1.2.3", get_next_version(&config,VersionBumpBehavior::Auto, &commits_since_last_tag, Some("v1.2.3"),false).unwrap());
    }
    #[test]
    fn error_if_release_but_no_commits_and_no_tag(){
        let commits_since_last_tag = vec![];
        let config =  get_config();

        assert!(get_next_version(&config,VersionBumpBehavior::Auto, &commits_since_last_tag, None ,true).is_err());
    }
    #[test]
    fn return_last_tag_if_no_commits(){
        let commits_since_last_tag = vec![];
        let config =  get_config();

        assert_eq!("1.2.3", get_next_version(&config,VersionBumpBehavior::Auto, &commits_since_last_tag, Some("v1.2.3") ,true).unwrap());
    }
    #[test]
    fn bump_major_auto(){
        let commits_since_last_tag = vec![
            "feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(), 
            "fix: message".to_string(), 
            "feat: message\n\n".to_string()];
        let config =  get_config();

        assert_eq!("2.0.0", get_next_version(&config,VersionBumpBehavior::Auto, &commits_since_last_tag, Some("v1.2.3"),true).unwrap());
        assert_eq!("2.0.0-alpha.3", get_next_version(&config,VersionBumpBehavior::Auto, &commits_since_last_tag, Some("v1.2.3"),false).unwrap());
    }
    #[test]
    fn bump_minor_auto(){
        let commits_since_last_tag = vec![
            "docs: messsage\n\ndesc\n\ncloses #5".to_string(),
            "fix: message".to_string(),
            "feat: message\n\n".to_string()];
        let config =  get_config();

        assert_eq!("1.3.0", get_next_version(&config,VersionBumpBehavior::Auto, &commits_since_last_tag, Some("v1.2.3"),true).unwrap());
        assert_eq!("1.3.0-alpha.3", get_next_version(&config,VersionBumpBehavior::Auto, &commits_since_last_tag, Some("v1.2.3"),false).unwrap());
    }
    #[test]
    fn bump_patch_auto(){
        let commits_since_last_tag = vec![
            "docs: messsage\n\ndesc\n\ncloses #5".to_string(),
            "fix: message".to_string(),
            "fix: message\n\n".to_string()];
        let config =  get_config();

        assert_eq!("1.2.4", get_next_version(&config,VersionBumpBehavior::Auto, &commits_since_last_tag, Some("v1.2.3"),true).unwrap());
        assert_eq!("1.2.4-alpha.3", get_next_version(&config,VersionBumpBehavior::Auto, &commits_since_last_tag, Some("v1.2.3"),false).unwrap());
    }
    #[test]
    fn no_bump(){
        let commits_since_last_tag = vec![
            "docs: messsage\n\ndesc\n\ncloses #5".to_string(),
            "fix: message".to_string(),
            "fix: message\n\n".to_string()];
        let config =  get_config();

        assert_eq!("1.2.3", get_next_version(&config,VersionBumpBehavior::None, &commits_since_last_tag, Some("v1.2.3"),true).unwrap());
        assert_eq!("1.2.3", get_next_version(&config,VersionBumpBehavior::None, &commits_since_last_tag, Some("v1.2.3"),false).unwrap());
    }
    #[test]
    fn bump_minor_auto_out_of_order(){
        let commits_since_last_tag = vec![
            "docs: messsage\n\ndesc\n\ncloses #5".to_string(),
            "fix: message".to_string(),
            "feat: message\n\n".to_string(),
            "fix: message".to_string(),];
        let config =  get_config();

        assert_eq!("1.3.0", get_next_version(&config,VersionBumpBehavior::Auto, &commits_since_last_tag, Some("v1.2.3"),true).unwrap());
        assert_eq!("1.3.0-alpha.4", get_next_version(&config,VersionBumpBehavior::Auto, &commits_since_last_tag, Some("v1.2.3"),false).unwrap());
    }
    #[test]
    fn bump_patch_if_there_are_commits_even_if_no_match(){
        let commits_since_last_tag = vec![
            "docs: messsage\n\ndesc\n\ncloses #5".to_string(),
            "test: message".to_string(),
            "test: message\n\n".to_string()];
        let config =  get_config();

        assert_eq!("1.2.4", get_next_version(&config,VersionBumpBehavior::Auto, &commits_since_last_tag, Some("v1.2.3"),true).unwrap());
        assert_eq!("1.2.4-alpha.3", get_next_version(&config,VersionBumpBehavior::Auto, &commits_since_last_tag, Some("v1.2.3"),false).unwrap());
    }
    #[test]
    fn bump_manual_pre(){
        let commits_since_last_tag = vec![
            "feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(), 
            "fix: message".to_string(),
            "feat: message\n\n".to_string()];
        let config =  get_config();

        assert_eq!("2.0.0-alpha.3", get_next_version(&config,VersionBumpBehavior::Major, &commits_since_last_tag, Some("v1.2.3"),false).unwrap());
        assert_eq!("1.3.0-alpha.3", get_next_version(&config,VersionBumpBehavior::Minor, &commits_since_last_tag, Some("v1.2.3"),false).unwrap());
        assert_eq!("1.2.4-alpha.3", get_next_version(&config,VersionBumpBehavior::Patch, &commits_since_last_tag, Some("v1.2.3"),false).unwrap());
    }
    #[test]
    fn bump_manual_release(){
        let commits_since_last_tag = vec![
            "feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(), 
            "fix: message".to_string(),
            "feat: message\n\n".to_string()];
        let config =  get_config();

        assert_eq!("2.0.0", get_next_version(&config,VersionBumpBehavior::Major, &commits_since_last_tag, Some("v1.2.3"),true).unwrap());
        assert_eq!("1.3.0", get_next_version(&config,VersionBumpBehavior::Minor, &commits_since_last_tag, Some("v1.2.3"),true).unwrap());
        assert_eq!("1.2.4", get_next_version(&config,VersionBumpBehavior::Patch, &commits_since_last_tag, Some("v1.2.3"),true).unwrap());
    }
    #[test]
    fn empty_tag_is_0(){
        let commits_since_last_tag = vec![
            "feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(),
            "fix: message".to_string(),
            "feat: message\n\n".to_string()];
        let config =  get_config();

        assert_eq!("0.1.0", get_next_version(&config,VersionBumpBehavior::Minor, &commits_since_last_tag, None,true).unwrap());
        assert_eq!("0.1.0-alpha.3", get_next_version(&config,VersionBumpBehavior::Minor, &commits_since_last_tag, None,false).unwrap());
    }
    #[test]
    fn unmatched_tag_is_0(){
        let commits_since_last_tag = vec![
            "feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(),
            "fix: message".to_string(),
            "feat: message\n\n".to_string()];
        let config =  get_config();

        assert_eq!("0.1.0", get_next_version(&config,VersionBumpBehavior::Minor, &commits_since_last_tag, Some("feature-tag"),true).unwrap());
        assert_eq!("0.1.0-alpha.3", get_next_version(&config,VersionBumpBehavior::Minor, &commits_since_last_tag, Some("feature-tag"),false).unwrap());
    }
    #[test]
    fn bump_removes_meta(){ // not sure if needed
        let commits_since_last_tag = vec![
            "feat: messsage\n\ndesc\n\nBREAKING CHANGE: some breaking change".to_string(),
            "fix: message".to_string(),
            "feat: message\n\n".to_string()];
        let config =  get_config();

        assert_eq!("1.2.4", get_next_version(&config,VersionBumpBehavior::Patch, &commits_since_last_tag, Some("1.2.3-beta.11+commitsha"),true).unwrap());
        assert_eq!("1.2.4-alpha.3", get_next_version(&config,VersionBumpBehavior::Patch, &commits_since_last_tag, Some("1.2.3-beta.11+commitsha"),false).unwrap());
    }
}
