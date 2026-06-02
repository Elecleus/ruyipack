pub mod targets;

pub use targets::json::from_json_file;
pub use targets::kcl::from_kcl_file;

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
