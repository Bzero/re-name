use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};
use std::env;
use std::fs;
use std::io::Error;
use std::path::Path;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    if env::var("PROFILE").unwrap() == "release" {
        let out_dir = Path::new("completions");
        fs::create_dir_all(&out_dir).unwrap();

        let mut cmd = Options::command();
        cmd.build();

        for &shell in Shell::value_variants() {
            generate_to(shell, &mut cmd, "re-name", &out_dir)?;
        }
    }

    Ok(())
}
