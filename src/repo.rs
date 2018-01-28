use libcalcver;

pub struct RepositoryConfig {
    pub scm_type: String,
    pub path: String
}

pub trait CreatableRepository {
    fn from(path: &str) -> Self;
}

impl RepositoryConfig {
    pub fn get_repo <T: libcalcver::repository::Repository + CreatableRepository>(&self) -> T {
        T::from(&self.path)
    }
}