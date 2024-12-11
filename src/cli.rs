use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "re-name", author, version, about, max_term_width = 128)]
#[clap(disable_help_flag = true, disable_version_flag = true)]
pub struct Options {
    #[arg(id = "SOURCE", required = true, help = "The regex pattern matching the source files.")]
    pub source: String,

    #[arg(id = "DESTINATION", required = true, help = "The pattern matching the destination files.")]
    pub destination: String,

    #[arg(short = 'p', long = "preview", help = "Show preview without renaming any files.")]
    //        alias = "dry-run",
    pub preview: bool,

    #[arg(short = 'v', long = "verbose", help = "Display what is being done.")]
    pub verbose: bool,

    #[arg(short = 'f', long = "force", help = "Overwrite files if they exist already.")]
    pub force: bool,

    #[arg(short='h', long="help", action=clap::ArgAction::Help, help="Print help.", required=false)]
    pub print_help: (),

    #[arg(short='V', long="version", action=clap::ArgAction::Version, help="Print version.", required=false)]
    pub print_version: (),
}
