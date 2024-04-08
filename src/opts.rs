use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "csv-cli", version, author, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// Show CSV or Convert CSV to JSON
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// Input file path
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    /// Output file path
    #[arg(short, long, value_parser = verify_output_file)]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    /// CSV has header or not
    #[arg(long, default_value_t = true)]
    header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exists.")
    }
}

fn verify_output_file(filename: &str) -> Result<String, String> {
    if filename.ends_with(".json") {
        // if Path::new(filename).extension().unwrap() == "json" {
        Ok(filename.into())
    } else {
        Err("Output files's extension should be `.json`.".into())
    }
}
