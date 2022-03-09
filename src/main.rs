mod metrics;
mod actions;
mod monitor;
mod storage;
mod config;
mod utils;

use tokio;
use dotenv::dotenv;
use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "sauron")]
#[clap(about = "HTTP service health watcher", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a one-off check
    #[clap(setting(AppSettings::DisableHelpFlag))]
    Run,
    /// Watch
    #[clap(setting(AppSettings::DisableHelpFlag))]
    Watch,
    /// Add thing
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Add {
        /// Stuff to add
        #[clap(required = true)]
        target: String,
        metric: String,
    },
    /// Remove thing
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Remove {
        /// Stuff to remove
        #[clap(required = true)]
        target: String,
    },
    /// List things
    #[clap(setting(AppSettings::DisableHelpFlag))]
    List,
    // Setup configuration
    #[clap(setting(AppSettings::DisableHelpFlag))]
    Setup,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Cli::parse();

    match &args.command {
        Commands::Run => {
            actions::check().await;
        }
        Commands::Watch => {
            actions::watch().await;
        }
        Commands::Add { target, metric } => {
            actions::add(target, metric)
        }
        Commands::Remove { target } => {
            actions::remove(target)
        }
        Commands::List => {
            actions::list()
        }
        Commands::Setup => {
            actions::setup()
        }
    };
}