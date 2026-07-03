use std::{fs::File, path::Path};

use ruyipack_core::package::PackageStatic;
use serde_json::from_reader;

use crate::input::InputError;

pub fn from_json_file(path: &Path) -> Result<PackageStatic, InputError> {
    let file = File::open(path)?;
    let result = from_reader(file)?;

    Ok(result)
}
