use std::env::VarError;

pub mod kcl;
pub mod rpm_spec;

#[derive(Debug)]
pub enum RepoError {
    EnvNotSet(VarError),
}

impl From<VarError> for RepoError {
    fn from(value: VarError) -> Self {
        Self::EnvNotSet(value)
    }
}
