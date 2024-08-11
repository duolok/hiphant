
use std::io::{self, Read};
use anyhow::{Result, Context};
use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Options {

    #[clap(default_value="Memory overload")]
    message: String,

    #[clap(short = 'c', long="cool-eyes")]
    /// Change eyes of the animal
    cool_eyes: String,

    #[clap(short='f', long="file")]
    /// Add a path to a file
    elephant_file: Option<std::path::PathBuf>,

    #[clap(short='i', long="stdin")]
    /// Read the message from STDIN intead of the argument
    stdin: bool,
}

fn process_input(options: &Options, message: &mut String) -> Result<()> {
    match options.stdin {
        true => { io::stdin().read_to_string(message).context("Failed read from stdin.")?;},
        false => { *message = options.message.clone() },
    }
    Ok(())
}

fn main() -> Result<()> {
    let options = Options::parse();
    let mut message = String::new();

    process_input(&options, &mut message)?;

    println!("{}", &message);

    Ok(())
}
