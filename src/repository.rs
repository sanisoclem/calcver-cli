

pub struct RepositoryConfig {
    pub scm_type: String,
    pub path: String
}

impl RepositoryConfig {
    pub fn get_repo <T: FileSystemRepository + CodeRepository>(&self) -> T {
        T::from(&self.path)
    }
}
// -- work around
pub trait FileSystemRepository {
    fn from(path: &str) -> Self;
}
pub trait CodeRepository {
    fn commit(&self, tag: &str);
    fn get_last_tag(&self) -> Option<&str>;
    fn get_commits_since_last_tag(&self) -> &Vec<String>;
}