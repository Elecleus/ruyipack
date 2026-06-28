pub mod targets;

use std::path::PathBuf;

use ruyi_core::package::PackageStatic;
pub use targets::json::from_json_file;
pub use targets::kcl::from_kcl_file;
pub use targets::stdin::from_stdin;

use crate::config::get_kcl_store;

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
        let first_letter = target.chars().nth(0).expect("Got empty package name");
        let target_path = get_kcl_store()
            .join(first_letter.to_string())
            .join(target.to_owned() + ".k");
        if target_path.exists() {
            return InputType::Name(target_path);
        }

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
