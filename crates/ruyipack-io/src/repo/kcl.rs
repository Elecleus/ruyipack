use std::{
    env::var,
    path::{Path, PathBuf},
};

use ruyipack_core::repo::Repo;

use crate::repo::RepoError;

pub struct KclRepo {
    path: PathBuf,
}

impl KclRepo {
    pub fn new() -> Result<Self, RepoError> {
        let path = {
            let store_path = var("KCL_STORE")?;
            PathBuf::from(store_path)
        };

        Ok(Self { path })
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }
}

impl Repo for KclRepo {
    fn get_by_name(&self, name: &str) -> Option<PathBuf> {
        let first_letter = name.chars().nth(0).expect("[Error] Got empty package name");
        let target_path = self
            .get_path()
            .to_owned()
            .join(first_letter.to_string())
            .join(name.to_owned() + ".k");

        if target_path.exists() {
            Some(target_path)
        } else {
            None
        }
    }
}
