# Hiphant

Hiphant is a fun command-line application that prints ASCII art elephants with customizable features. This project is what happens on a Sunday when I'm bored.

## Features

- Display different elephant ASCII art designs.
- Customize the eye style of the elephants.
- Store elephant templates in a user-specific directory for easy management.

## Installation

To install Hiphant, you need to have Rust and Cargo installed on your system. If they are not installed, you can follow the [Rust installation guide](https://www.rust-lang.org/tools/install).

### Building from Source

1. Clone the repository:

```bash
   git clone https://github.com/yourusername/hiphant.git
```

2. Build the project:

```bash
   cargo build --release
```

Install the binary globally:

```bash
   cargo install --path ./
```
Copy the elephant ASCII art files to the configuration directory:

Copy code
```bash
   mkdir -p ~/.config/hiphant/animals
   cp animals/*.txt ~/.config/hiphant/animals/

```
### Usage
Once installed, you can run Hiphant from the command line:

```bash
   hiphant [OPTIONS]

   Options

   -n, --number <NUMBER>: Specify the elephant design number (e.g., 1, 2, 3).
   -s, --smile: Use a smiling eye for the elephant.
   -m, --message <MESSAGE>: Display a custom message with the elephant.
   -f, --file <FILE>: Specify a custom path to an elephant template file.
   -i, --stdin: Read the message from standard input instead of an argument.
```
