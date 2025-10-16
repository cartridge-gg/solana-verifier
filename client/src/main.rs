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

fn main() {
    let result = std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            env_logger::Builder::from_default_env()
                .filter_level(log::LevelFilter::Info)
                .init();

            let rt = tokio::runtime::Builder::new_multi_thread()
                .thread_stack_size(32 * 1024 * 1024)
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async_main())
        })
        .unwrap()
        .join()
        .unwrap();

    if let Err(e) = result {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}

#[allow(clippy::result_large_err)]
async fn async_main() -> client::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Subcommands::Deploy(config) => deploy::deploy(&config).await?,
        Subcommands::Verify(config) => verify::verify(&config).await?,
        Subcommands::RetriveFunds(config) => retrive_funds::retrive_funds(&config).await?,
    }
    Ok(())
}