use crate::package::PackageStatic;

pub trait Repo {
    // [TODO] change the return type to an IR.
    fn get_name(name: &str) -> PackageStatic;
}
