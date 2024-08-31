mod config;
mod ssh_client;
mod command_executor; // Add this line
mod output; // Add this line

use clap::Parser;
use dotenv::dotenv;
use tracing::{error, Level};
use tracing_subscriber::EnvFilter;
use std::env;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Parser)]
struct Args {
    #[clap(long, default_value = "PlainText")]
    output_format: String,

    #[clap(long, default_value = "output.txt")]
    output_file: String,

    #[clap(long, default_value = "prod")]
    group: String,

    #[clap(long, default_value_t = 3)]
    retries: u32,

    #[clap(long)]
    command_file: Option<String>,
}

#[derive(Deserialize)]
struct CommandList {
    commands: Vec<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Set up the logging subscriber with a higher log level to show only errors
    tracing_subscriber::fmt()
        .with_max_level(Level::ERROR)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    // Load commands from the specified JSON file
    let commands = match &args.command_file {
        Some(command_file) => {
            let mut file = File::open(command_file).expect("Failed to open command file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Failed to read command file");
            let command_list: CommandList = serde_json::from_str(&contents).expect("Failed to parse command file");
            command_list.commands
        },
        None => {
            error!("No command file specified.");
            return;
        }
    };

    let username = env::var("SSH_USERNAME").expect("SSH_USERNAME not set");
    let password = env::var("SSH_PASSWORD").expect("SSH_PASSWORD not set");

    // Execute commands on the hosts in the specified group
    let results = command_executor::distributed_execute(
        &args.group, "config.json", &username, &password, commands, args.retries
    ).await;

    output::save_results(results, &args.output_format, &args.output_file);
}
