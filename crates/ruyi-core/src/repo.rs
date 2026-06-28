use std::path::PathBuf;

pub trait Repo {
    // [TODO] change the return type to an IR.
    fn get_by_name(&self, name: &str) -> Option<PathBuf>;
}
