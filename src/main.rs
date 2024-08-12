use std::io::{self, Read};
use anyhow::{Result, Context};
use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Options {

    #[clap(default_value="Memory overload")]
    message: String,

    #[clap(short='h', long="happy")]
    /// Give animal happy eyes
    happy: bool,

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

    let happy_eye = if options.happy { "^" } else { "o" };
    
    match &options.elephant_file {
        Some(path) => {
            let elephant_templatae = std::fs::read_to_string(path).with_context(
                || format!("Could not read file {:?}", path))?;

            let eye = format!("{}", happy_eye.blue().bold());
            let _elephant_picture = elephant_templatae.replace("{eye}", &eye);
            println!(
                "{}",
                message.bright_green().underline().on_cyan()
                );
            println!("{}", &_elephant_picture);
        }, 
        None => {

    }
    }


    Ok(())
}

fn main() -> Result<()> {
    let options = Options::parse();
    let mut message = String::new();

    process_input(&options, &mut message)?;

    println!("{}", &message.on_red());

    Ok(())
}
