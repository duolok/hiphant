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

    #[clap(short ='n', long="number", default_value="1")]
    /// Elephant number
    elephant_number: usize,

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

fn print_animal(eye: &str, elephant_number: usize) -> Result<()> {
    let filename = format!("animals/elephant_{}.txt", elephant_number);
    let elephant_template = std::fs::read_to_string(&filename)
        .with_context(|| format!("Could not read file {}", filename))?;

    let elephant_picture = elephant_template.replace("{eye}", eye);
    println!(" \\");
    println!("  \\");
    println!("   \\");
    println!("{}", elephant_picture);
    
    Ok(())

}

fn main() -> Result<()> {
    let options = Options::parse();
    let mut message = String::new();
    let happy_eye = if options.happy { "^" } else { "@" };

    process_input(&options, &mut message)?;

    match &options.elephant_file {
        Some(path) => {
            let elephant_template = std::fs::read_to_string(path).with_context(|| {
                format!("Could not read file {:?}", path)
            })?;

            let eye = format!("{}", happy_eye.blue().bold().on_cyan());
            let elephant_picture = elephant_template.replace("{eye}", &eye);

            println!(
                "{}",
                message.bright_yellow().underline().on_purple()
            );

            println!("{}", &elephant_picture);
        }
        None => {
            println!("{}", message.bright_yellow().underline().on_purple());
            let eye = format!("{}", happy_eye.blue().bold().on_cyan());
            print_animal(&eye, options.elephant_number)?;
        }
    }

    Ok(())
}
