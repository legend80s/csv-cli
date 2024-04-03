use clap::Parser;
use csv_cli::{save_as_json, Cli, SubCommands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        SubCommands::Csv(opts) => save_as_json(opts.input, opts.output)?,
    }

    Ok(())
}
