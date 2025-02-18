use clap::{Command, CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};
use clap_mangen::Man;
use std::env;
use std::fs;
use std::io::Error;
use std::path::Path;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    if env::var("PROFILE").unwrap() == "release" {
        let mut cmd = Options::command();
        cmd.build();

        generate_completions(&mut cmd)?;
        generate_man(cmd)?;
    }

    Ok(())
}

fn generate_completions(cmd: &mut Command) -> Result<(), Error> {
    let out_dir = Path::new("completions");
    fs::create_dir_all(&out_dir).unwrap();

    for &shell in Shell::value_variants() {
        generate_to(shell, cmd, "re-name", &out_dir)?;
    }
    Ok(())
}

fn generate_man(cmd: Command) -> Result<(), Error> {
    let out_dir = Path::new("man");
    fs::create_dir_all(&out_dir).unwrap();

    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(out_dir.join("re-name.1"), buffer)?;
    Ok(())
}
