use anyhow::*;
use XML_to_JSON_Parser::*;
use clap::*;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse { file: String },
    Instruction,
    Credits,
}

fn main() -> anyhow::Result<()> {
    
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { file } => {
            let xml = fs::read_to_string(&file).with_context(|| format!("Error reading file"))?;
            let json = start_parser(&xml)?;
            println!("{}", serde_json::to_string_pretty(&json)?);
        }

        Commands::Instruction => {
            let text = r#"
                Welcome to the XML_To_JSON_Parser!
                
                To use this parser you will need file with XML language content in .txt or .xml format. Make sure your XML language content is valid. 
                Put this file in the directory.
                To start parser open terminal and write command cargo run -- parse your_file_name.xml. 
                As a result program will print parsed XML into JSON.
            "#;
            println!("{}", text);
        }

        Commands::Credits => {
            let text = r#"
                XML_To_JSON_Parser
                Author: Artur Nozhenko
                2025
            "#;
            println!("{}", text);
        }
    }
    Ok(())
}