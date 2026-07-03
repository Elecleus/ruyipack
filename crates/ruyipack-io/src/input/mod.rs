pub mod targets;

use std::path::PathBuf;

use ruyipack_core::package::PackageStatic;
use ruyipack_core::repo::Repo;
pub use targets::json::from_json_file;
pub use targets::kcl::from_kcl_file;
pub use targets::stdin::from_stdin;

use crate::repo::{kcl::KclRepo, rpm_spec::RpmSpecRepo};

pub fn input_router(target: &str) -> PackageStatic {
    match InputType::paste(target) {
        InputType::File(path_buf) => {
            let ext = path_buf
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("");
            match ext {
                "k" => from_kcl_file(&path_buf),
                "json" => from_json_file(&path_buf),
                _ => {
                    panic!("[Error] Failed to infer FILE type.")
                }
            }
        }
        InputType::Name(path_buf) => from_kcl_file(&path_buf),
        InputType::Stdin => from_stdin(),
    }
    .unwrap()
}

pub enum InputType {
    File(PathBuf),
    Name(PathBuf),
    Stdin,
}

impl InputType {
    fn paste(target: &str) -> InputType {
        // Check for File.
        let target_path = PathBuf::from(target);
        if target_path.exists() {
            return InputType::File(target_path);
        }

        // Check for Name.
        if let Ok(repo) = KclRepo::new() {
            if let Some(target_path) = repo.get_by_name(target) {
                return InputType::Name(target_path);
            }
        } // [TODO] else log: not found in kcl store.
        if let Ok(repo) = RpmSpecRepo::new() {
            if let Some(target_path) = repo.get_by_name(target) {
                return InputType::Name(target_path);
            }
        } // [TODO] else log: not found in kcl store.

        // Stdin
        eprintln!("[INFO] Fallback to read from stdin.");
        return InputType::Stdin;
    }
}

#[derive(Debug)]
pub enum InputError {
    FileOpenError(std::io::Error),
    DeserializeError(serde_json::Error),
    CommandRunningError(std::io::Error),
}

impl From<std::io::Error> for InputError {
    fn from(value: std::io::Error) -> Self {
        InputError::FileOpenError(value)
    }
}

impl From<serde_json::Error> for InputError {
    fn from(value: serde_json::Error) -> Self {
        InputError::DeserializeError(value)
    }
}
