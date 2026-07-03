use askama::Template;

use ruyipack_core::package::{BuildStep, PackageStatic};

// 此部分字段直接对应模板填充
#[derive(Template, Default)]
#[template(path = "rpm-template.spec", escape = "none")]
pub struct RpmSpecTemplate {
    name: String,
    version: String,
    release: String,
    license: String,
    url: String,
    vcs: String,
    build_system: String,
    build_steps: Vec<BuildStepTemplate>,
    sources: Vec<SourceTemplate>,
    output_main: OutputTemplate,
    output_others: Vec<OutputTemplate>,
    changelog: Option<String>,
}

#[derive(Default)]
struct BuildStepTemplate {
    script: String,
    environment: String,
}

#[allow(unused)]
#[derive(Default)]
struct SourceTemplate {
    name: String,
    type_: String,
    url: String,
    checksum: String,
    checksum_type: String,
}

#[derive(Default)]
struct OutputTemplate {
    name: String,
    summary: String,
    description: String,
    requires: String,
    build_requires: String,
    files: String,
}

impl TryFrom<&PackageStatic> for RpmSpecTemplate {
    type Error = IntoRpmSpecTemplateError;

    fn try_from(value: &PackageStatic) -> Result<Self, Self::Error> {
        let output_main: OutputTemplate =
            if let Some(main_output) = value.outputs.get_key_value("main") {
                OutputTemplate {
                    name: main_output.0.clone(),
                    summary: main_output.1.summary.clone(),
                    description: main_output.1.description.clone(),
                    requires: main_output.1.requires.join("\n"),
                    build_requires: main_output.1.build_requires.join("\n"),
                    files: main_output.1.files.join("\n"),
                }
            } else {
                return Err(IntoRpmSpecTemplateError::MainOutputNotExist);
            };

        let output_others: Vec<OutputTemplate> = value
            .outputs
            .iter()
            .filter(|o| o.0 != "main")
            .map(|output| OutputTemplate {
                name: output.0.clone(),
                summary: output.1.summary.clone(),
                description: output.1.description.clone(),
                requires: output.1.requires.join("\n"),
                build_requires: output.1.build_requires.join("\n"),
                files: output.1.files.join("\n"),
            })
            .collect();

        let build_steps: Vec<BuildStepTemplate> =
            value.build_steps.iter().map(Into::into).collect();

        let sources: Vec<SourceTemplate> = value
            .sources
            .iter()
            .map(|(name, source)| SourceTemplate {
                name: name.clone(),
                type_: source.type_.clone(),
                url: source.url.clone(),
                checksum: source.checksum.clone(),
                checksum_type: source.checksum_type.clone(),
            })
            .collect();

        Ok(Self {
            name: value.name.clone(),
            version: value.version.clone(),
            release: value.release.clone(),
            license: value.license.clone(),
            url: value.url.clone(),
            vcs: value.vcs.clone(),
            build_system: value.build_system.clone(),
            build_steps,
            sources,
            output_main,
            output_others,
            changelog: None,
        })
    }
}

impl From<&BuildStep> for BuildStepTemplate {
    fn from(value: &BuildStep) -> Self {
        Self {
            script: value.script.clone(),
            environment: value
                .environment
                .iter()
                .map(|(k, v)| format!("{} = {}", k, v))
                .collect(),
        }
    }
}

#[derive(Debug)]
pub enum IntoRpmSpecTemplateError {
    MainOutputNotExist,
    FilesNotExist,
}
