
use std::io::{self, Read};
use anyhow::{Result, Context};
use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Options {

    #[clap(default_value="Memory overload")]
    message: String,

    #[clap(short='f', long="file")]
    /// Add a path to a file
    elephant_file: Option<std::path::PathBuf>,

    #[clap(short='i', long="stdin")]
    /// Read the message from STDIN intead of the argument
    stdin: bool,
}




fn main() -> Result<()> {
    let options = Options::parse();
    let message = options.message;

    println!("{}", &message);

    Ok(())
}
