use clap::Parser;
use std::path::PathBuf;
use std::time::Duration;

/// Solana program client configuration
#[derive(Parser, Debug)]
#[clap(author, version, about = "Solana Program Client")]
pub struct Config {
    /// RPC URL for the Solana cluster
    #[clap(long, default_value = "http://localhost:8899")]
    pub rpc_url: String,

    /// RPC timeout in seconds
    #[clap(long, default_value = "30")]
    pub rpc_timeout_secs: u64,

    /// Path to the program binary
    #[clap(long, default_value = "target/deploy/greeting.so")]
    pub program_path: PathBuf,

    /// Path to the payer keypair file
    #[clap(long, default_value = "payer-keypair.json")]
    pub payer_keypair_path: PathBuf,

    /// Path to the program keypair file
    #[clap(long, default_value = "program-keypair.json")]
    pub program_keypair_path: PathBuf,

    /// Path to the greeting account keypair file
    #[clap(long, default_value = "greeting-keypair.json")]
    pub greeting_keypair_path: PathBuf,

    /// Directory for data files
    #[clap(long, default_value = "data")]
    pub data_dir: PathBuf,

    /// Amount of SOL to airdrop initially
    #[clap(long, default_value = "2000000000")]
    pub airdrop_amount: u64,

    /// Multiplier for additional airdrop
    #[clap(long, default_value = "5")]
    pub additional_airdrop_multiplier: u64,

    /// Number of transaction retry attempts
    #[clap(long, default_value = "10")]
    pub transaction_retry_count: usize,

    /// Sleep duration between retries in seconds
    #[clap(long, default_value = "1")]
    pub retry_sleep_secs: u64,

    /// Buffer chunk size for program deployment
    #[clap(long, default_value = "900")]
    pub buffer_chunk_size: usize,
}

impl Config {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Get the retry sleep duration
    pub fn retry_sleep_duration(&self) -> Duration {
        Duration::from_secs(self.retry_sleep_secs)
    }

    /// Get the RPC timeout duration
    pub fn rpc_timeout_duration(&self) -> Duration {
        Duration::from_secs(self.rpc_timeout_secs)
    }
}
