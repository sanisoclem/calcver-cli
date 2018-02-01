use libcalcver;

pub struct RepositoryConfig {
    pub scm_type: String,
    pub path: String
}

pub trait CodeRepository {
    fn from(path: &str) -> Self;
    fn commit(&self, tag: &str);
}

impl RepositoryConfig {
    pub fn get_repo <T: libcalcver::repository::Repository + CodeRepository>(&self) -> T {
        T::from(&self.path)
    }
}