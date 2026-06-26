use std::{env::var, path::PathBuf};

pub fn get_kcl_store() -> PathBuf {
    let store_path = var("KCL_STORE").expect("[Error] Environment variable KCL_STORE not set");

    PathBuf::from(store_path)
}
