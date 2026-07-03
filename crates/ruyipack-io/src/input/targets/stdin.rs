use std::io;

use ruyipack_core::package::PackageStatic;
use serde_json::from_reader;

use crate::input::InputError;

pub fn from_stdin() -> Result<PackageStatic, InputError> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let result = from_reader(handle)?;

    Ok(result)
}
