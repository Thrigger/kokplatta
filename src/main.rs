use std::path::PathBuf;
use std::fs;

use clap::Parser;
use serde_derive::Deserialize;

/// Template application for reducing the amount of boilerplate
/// This is an introduction to the application and a small description on what it does.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: PathBuf,

    /// Enable verbose mode. 
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Emojis can be flags as well
    #[arg(short='😊', long)]
    happy: bool,
}

/// Struct for the Toml file to parse
#[derive(Deserialize,Debug)]
struct ConfigFile {
    /// Name is mandatory in the Toml file
    name: String,
    /// Optional variable
    name2: Option<String>,
    ///Mandatory sub struct
    header: Header,
}

/// Sub struct for a header in the Toml file
#[derive(Deserialize,Debug)]
struct Header{
    value_int: u8,
    value_int2: Option<u8>,
    value_str: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let config_path = &cli.config;
    println!("Value for config: {}", config_path.display());

    if cli.happy {
        println!("The emoji flag is true");
    } else {
        println!("Not happy");
    }

    match cli.verbose {
        0 => println!("Debug mode is off"),
        1 => println!("Warnings are shown"),
        2 => println!("All information is shown"),
        _ => println!("Meh..."),
    }

    let contents = fs::read_to_string(cli.config).unwrap();
    let conf_file: ConfigFile = toml::from_str(&contents).expect("Parse toml file");
    println!("toml-file as config_file:\n{:#?}", conf_file);

    // Continued program logic goes here...
}
