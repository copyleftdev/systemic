use tokio::task;
use futures::future::join_all;
use std::sync::Arc;
use crate::ssh_client;
use crate::config;
use prettytable::{Table, row}; // Import only necessary macros

/// Executes commands across multiple hosts and aggregates results.
pub async fn distributed_execute(
    host_group: &str, 
    config_path: &str, 
    username: &str, 
    password: &str, 
    commands: Vec<String>,
    retries: u32
) -> Vec<String> {
    // Load configuration and filter hosts based on the group key name
    let config = config::load_config(config_path).expect("Failed to load configuration");
    let hosts = config::filter_hosts(&config, host_group);
    
    if hosts.is_empty() {
        eprintln!("No hosts found for the group key: {}", host_group);
        return vec![];
    }

    let username = Arc::new(username.to_string());
    let password = Arc::new(password.to_string());
    let commands = Arc::new(commands);

    // Prepare tasks for parallel execution
    let tasks: Vec<_> = hosts.into_iter().map(|host| {
        let username = Arc::clone(&username);
        let password = Arc::clone(&password);
        let commands = Arc::clone(&commands);
        
        task::spawn(async move {
            let mut results = Vec::new();
            for command in commands.iter() {
                let mut attempt = 0;
                while attempt < retries {
                    match ssh_client::ssh_execute(&host, &username, &password, command).await {
                        Ok(result) => {
                            results.push(result);
                            break;
                        },
                        Err(err) => {
                            attempt += 1;
                            if attempt >= retries {
                                results.push(format!("Host: {}\nCommand: {}\nStandard Error: {}", host, command, err));
                            }
                        }
                    }
                }
            }
            results
        })
    }).collect();

    // Collect all results from tasks
    let all_results: Vec<String> = join_all(tasks).await.into_iter()
        .flat_map(|r| r.unwrap_or_else(|_| vec!["Error".to_string()])) // Flatten the Vec<Vec<String>> into Vec<String>
        .collect();

    // Create a single table to display all host interactions without the "Unknown Command" entries
    let mut table = Table::new();
    table.add_row(row!["Host", "Command", "Standard Output", "Standard Error"]);

    for result in all_results.iter() {
        // Correctly parse the result format
        let lines: Vec<&str> = result.lines().collect();
        let host = extract_value(&lines, "Host: ");
        let command = extract_value(&lines, "Command: ");
        let stdout = lines.iter()
            .filter(|&&line| !line.starts_with("Host: ") && !line.starts_with("Command: ") && !line.starts_with("Standard Error: "))
            .cloned()
            .collect::<Vec<&str>>()
            .join("\n");
        let stderr = extract_value(&lines, "Standard Error: ");

        // Ensure that only valid command entries are added to the table
        if command != "Unknown Command" {
            table.add_row(row![host, command, stdout, stderr]);
        }
    }

    // Print the single consolidated table
    table.printstd();

    all_results
}

/// Helper function to extract values from lines.
fn extract_value(lines: &[&str], prefix: &str) -> String {
    lines.iter()
        .find(|&&line| line.starts_with(prefix))
        .map(|&line| line.replacen(prefix, "", 1))
        .unwrap_or_else(|| "Unknown".to_string())
}
