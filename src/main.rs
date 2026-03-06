use clap::{CommandFactory, Parser};

fn main() -> anyhow::Result<()> {
    let cli = devports::cli::Cli::parse();

    if cli.command.is_none() {
        devports::cli::Cli::command().print_long_help()?;
        println!();
        return Ok(());
    }

    devports::execute(cli)
}
