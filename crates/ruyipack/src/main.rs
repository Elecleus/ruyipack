use askama::Template as _;
use clap::{arg, Arg, Command};
use ruyipack_core::package::PackageStatic;
use ruyipack_io::{
    input::{self, input_router},
    output::targets::rpm_spec::RpmSpecTemplate,
};

fn main() -> Result<(), input::InputError> {
    let matches = Command::new("ruyi")
        .version("0.0.0")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("import")
                .about("Import a file as PackageStatic (JSON), only prints to stdout")
                .long_about("")
                .arg(arg!(<FILE>)),
        )
        .subcommand(
            Command::new("export")
                .about("Export PackageStatic as FORMAT")
                .arg(arg!([FILE]))
                .arg(
                    Arg::new("format")
                        .short('f')
                        .help("The format in which to export")
                        .value_parser(["rpm-spec"])
                        .default_value("rpm-spec"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("import", import_matches)) => {
            let target = import_matches
                .get_one::<String>("FILE")
                .expect("This should never panic.");

            // Load the file, according to the file format.
            let loaded: PackageStatic = input_router(target);

            println!("{}", serde_json::to_string_pretty(&loaded)?)
        }
        Some(("export", export_matches)) => {
            let target = export_matches.get_one::<String>("FILE");

            let loaded: PackageStatic = match target {
                Some(s) => input_router(s),
                None => input::from_stdin()?,
            };

            let rpm_spec = <&PackageStatic as TryInto<RpmSpecTemplate>>::try_into(&loaded)
                .unwrap()
                .render()
                .unwrap();
            println!("{}", rpm_spec);
        }
        _ => unreachable!(),
    }

    Ok(())
}
