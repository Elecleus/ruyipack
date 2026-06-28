use std::env::VarError;

pub mod kcl;

#[derive(Debug)]
pub enum RepoError {
    EnvNotSet(VarError),
}

impl From<VarError> for RepoError {
    fn from(value: VarError) -> Self {
        Self::EnvNotSet(value)
    }
}
