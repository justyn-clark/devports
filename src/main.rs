use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = ports::cli::Cli::parse();
    ports::execute(cli)
}
