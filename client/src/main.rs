use clap::{Parser, Subcommand};
use client::{deploy, retrive_funds, verify, Config};

#[derive(Debug, Parser)]
#[clap(about, version)]
struct Cli {
    #[clap(subcommand)]
    command: Subcommands,
}

#[derive(Debug, Subcommand)]
enum Subcommands {
    /// Verify a proof using existing account and programs
    Verify(Config),
    /// Deploy a new program to the solana and create a new account
    Deploy(Config),
    /// Retrive funds from the solana (close the account)
    RetriveFunds(Config),
}

#[tokio::main]
#[allow(clippy::result_large_err)]
async fn main() -> client::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Subcommands::Verify(config) => verify::verify(&config).await?,
        Subcommands::Deploy(config) => deploy::deploy(&config).await?,
        Subcommands::RetriveFunds(config) => retrive_funds::retrive_funds(&config).await?,
    }
    Ok(())
}
