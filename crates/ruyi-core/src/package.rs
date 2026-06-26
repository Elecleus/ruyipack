//! The standard static definition of a package.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageStatic {
    pub name: String,
    pub version: String,
    pub release: String,
    pub license: String,
    pub url: String,
    pub vcs: String,
    #[serde(alias = "buildSystem" /* For KCL */)]
    pub build_system: String,
    #[serde(default, alias = "build_steps" /* For KCL */)]
    pub build_steps: Vec<BuildStep>,
    pub sources: IndexMap<String, Source>,
    pub outputs: IndexMap<String, Output>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildStep {
    pub script: String,
    pub environment: IndexMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    #[serde(alias = "type" /* For KCL */)]
    pub type_: String,
    pub url: String,
    pub checksum: String,
    #[serde(alias = "checksumType" /* For KCL */)]
    pub checksum_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub summary: String,
    pub description: String,
    #[serde(default)]
    pub requires: Vec<String>,
    #[serde(default)]
    pub build_requires: Vec<String>,
    #[serde(default)]
    pub files: Vec<String>,
}
