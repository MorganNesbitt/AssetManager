use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("Asset Manager")
        .version("0.1")
        .author("humansnotfish")
        .subcommand(SubCommand::with_name("completions"))
        .subcommand(
            SubCommand::with_name("strip")
                .arg(
                    Arg::with_name("input")
                        .long("input")
                        .takes_value(true)
                        .required(true)
                        .help("input path to scan. Directory or file"),
                )
                .arg(
                    Arg::with_name("output")
                        .long("output")
                        .value_name("OUTPUT_DIRECTORY")
                        .takes_value(true)
                        .required(true)
                        .help("output directory to store spritesheet"),
                )
                .about("takes a path and strips the found images of transparency")
                .version("0.1"),
        )
        .subcommand(
            SubCommand::with_name("pack")
                .arg(
                    Arg::with_name("output")
                        .long("output")
                        .value_name("OUTPUT_DIRECTORY")
                        .takes_value(true)
                        .required(true)
                        .help("output directory to store spritesheet"),
                )
                .arg(
                    Arg::with_name("input")
                        .long("input")
                        .value_name("INPUT_DIRECTORY")
                        .takes_value(true)
                        .required(true)
                        .help("input directory to scan"),
                )
                .about("pack a directory of assets in a sprite sheet")
                .version("0.1"),
        )
}
