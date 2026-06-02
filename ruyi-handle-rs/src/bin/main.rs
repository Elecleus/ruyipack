use std::path::Path;

use askama::Template;
use clap::{Arg, ArgAction, Command, arg};
use ruyi_handle::input;

fn main() -> Result<(), input::InputError> {
    let matches = Command::new("ruyi")
        .version("0.0.0")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("build")
                .about("build a file into PackageStatic (JSON)")
                .long_about("")
                .arg(arg!(<FILE>))
                .arg(
                    Arg::new("recursive")
                        .short('r')
                        .long("recursive")
                        .help("[TODO]")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("debug")
                        .short('d')
                        .long("debug")
                        .help("[TODO] Showing re-exported JSON")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(Command::new("export"))
        .get_matches();

    match matches.subcommand() {
        Some(("build", build_matches)) => {
            if build_matches.get_flag("recursive") {
                println!("[Warning] recursive build not implemented.")
            }

            let path = Path::new(build_matches.get_one::<String>("FILE").unwrap());
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

            let imported = match ext {
                "k" => input::from_kcl_file(path)?,
                "json" => input::from_json_file(path)?,
                _ => {
                    panic!("[Error] Failed to infer FILE type.")
                }
            };

            let rpm_spec = imported.to_rpm_spec_template().unwrap().render().unwrap();

            if build_matches.get_flag("debug") {
                println!("{}", serde_json::to_string_pretty(&imported)?)
            };
            println!("{}", rpm_spec);

            // write("../temp/graphviz.spec", rpm_spec).unwrap();
        }
        _ => unreachable!(),
    }

    Ok(())
}
