use clap::{CommandFactory, Parser};

fn main() -> anyhow::Result<()> {
    let cli = devports::cli::Cli::parse();

    if cli.command.is_none() {
        println!("devports v{}", env!("CARGO_PKG_VERSION"));
        println!("Local development port manager\n");
        devports::cli::Cli::command().print_help()?;
        println!();
        return Ok(());
    }

    devports::execute(cli)
}
