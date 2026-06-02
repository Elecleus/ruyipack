use crate::input::InputError;
use serde_json::from_reader;

use crate::package::PackageStatic;

use std::{fs::File, path::Path};

pub fn from_json_file(path: &Path) -> Result<PackageStatic, InputError> {
    let file = File::open(path)?;
    let result = from_reader(file)?;

    Ok(result)
}
