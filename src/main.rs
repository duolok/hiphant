use std::io::{self, Read};
use anyhow::{Result, Context};
use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Memory overload")]
    /// Message to display with the elephant
    message: String,

    #[clap(short = 's', long = "smile")]
    /// Give animal happy eyes
    happy: bool,

    #[clap(short = 'f', long = "file")]
    /// Add a path to a file containing an elephant template
    elephant_file: Option<std::path::PathBuf>,

    #[clap(short = 'i', long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool,
}

fn process_input(options: &Options, message: &mut String) -> Result<()> {
    match options.stdin {
        true => {
            io::stdin()
                .read_to_string(message)
                .context("Failed to read from stdin.")?;
        }
        false => {
            *message = options.message.clone();
        }
    }
    Ok(())
}

fn print_animal(eye: &str) {
    println!(
"    _.-- ,.--. 
   .'   .'    /\\
   | {eye}       |'..--------._
  /      \\._/              '.
 /  .-.-                     \\
(  /    \\                     \\
 \\\\      '.                  | #
  \\\\       \\   -.           /
   :\\       |    )._____.'   \\
            |   /  \\  |  \\    )
            |   |./'  :__ \\.-'
            '--'
"
    );
}

fn main() -> Result<()> {
    let options = Options::parse();
    let mut message = String::new();

    // Process input from either stdin or default message
    process_input(&options, &mut message)?;

    let happy_eye = if options.happy { "^" } else { "@" };

    match &options.elephant_file {
        Some(path) => {
            let elephant_template = std::fs::read_to_string(path).with_context(|| {
                format!("Could not read file {:?}", path)
            })?;

            let eye = format!("{}", happy_eye.blue().bold().on_cyan());
            let elephant_picture = elephant_template.replace("{eye}", &eye);

            println!("{}", message.bright_green().underline().on_blue());
            println!("{}", &elephant_picture);
        }
        None => {
            println!("{}", message.bright_cyan().bold().on_black());
            println!(" \\");
            println!("  \\");
            println!("   \\");
            print_animal(&happy_eye);
        }
    }

    Ok(())
}

