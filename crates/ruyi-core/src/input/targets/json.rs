use std::{fs::File, path::Path};

use serde_json::from_reader;

use crate::{input::InputError, package::PackageStatic};

pub fn from_json_file(path: &Path) -> Result<PackageStatic, InputError> {
    let file = File::open(path)?;
    let result = from_reader(file)?;

    Ok(result)
}
