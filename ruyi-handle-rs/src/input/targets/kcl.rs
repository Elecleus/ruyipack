use std::{path::Path, process::Command};

use crate::{input::InputError, package::PackageStatic};

pub fn from_kcl_file(path: &Path) -> Result<PackageStatic, InputError> {
    let kcl_output = Command::new("kcl")
        .arg("run")
        .arg("--format")
        .arg("json")
        .arg(path)
        .output()?;

    if !kcl_output.status.success() {
        let stderr = String::from_utf8_lossy(&kcl_output.stderr);
        eprintln!("命令失败: {}", stderr);
        std::process::exit(1);
    }

    let result = serde_json::from_slice(&kcl_output.stdout)?;

    Ok(result)
}
