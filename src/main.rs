use std::collections::HashMap;
use std::process::Command;
use std::str;
use std::thread;
use std::time::Duration;

mod container;
mod error_history;

use container::{analyze_logs, parse_container_info};

fn main() {
    let output = Command::new("docker")
        .arg("ps")
        .arg("--format")
        .arg("{{.ID}}\t{{.Names}}")
        .output()
        .expect("failed to retrieve docker container list");

    let output_to_parse = str::from_utf8(&output.stdout).unwrap();
    let current_containers = parse_container_info(output_to_parse);

    // Create a HashMap to store the last checked log count for each container
    let mut last_checked_log_counts: HashMap<String, usize> = HashMap::new();

    // Create a HashMap to store the error history for each container
    let mut error_history: HashMap<String, Vec<(usize, String)>> = HashMap::new();

    // create a hashmap to store
    // total errors, errors in last 5 mins, errors in last 15 seconds
    let mut error_counts: HashMap<String, (usize, usize, usize)> = HashMap::new();

    // Define the maximum number of error entries to keep
    let max_error_entries = 10;

    // Define the maximum age of error entries to keep (e.g., 1 day)
    let max_error_age = Duration::from_secs(24 * 60 * 60);

    loop {
        for container in &current_containers {
            // update the container logs
            let logs = container::get_container_logs(&container.id);
            let container = container.with_logs(logs);
            match container.logs {
                Some(ref logs) => {
                    let container_id = &container.id;
                    // Get the last checked log count for this container
                    let last_checked_log_count = last_checked_log_counts
                        .entry(container_id.clone())
                        .or_insert(0);

                    // Fetch the current logs for this container
                    let current_logs = logs;

                    // Calculate the number of new log entries
                    let new_log_count = current_logs.lines().count() - *last_checked_log_count;
                    println!(
                        "container: {}, new log count: {}, last checked log count: {}",
                        container.name, new_log_count, *last_checked_log_count
                    );

                    if new_log_count > 0 {
                        // Display the new logs
                        let new_logs = &current_logs
                            .lines()
                            .skip(*last_checked_log_count)
                            .collect::<Vec<_>>();

                        // Update the last checked log count
                        *last_checked_log_count = current_logs.lines().count();

                        // Analyze new logs for errors
                        let joined_logs = new_logs.join("\n");
                        let errors = analyze_logs(&joined_logs);
                        if errors.len() > 0 {
                            println!(
                                "Errors found in container: {}, count: {}",
                                container.name,
                                errors.len()
                            );

                            // Update the error history
                            let current_time = chrono::Local::now().to_string();
                            let entry = (errors.len(), current_time);
                            let error_entries = error_history
                                .entry(container_id.clone())
                                .or_insert(Vec::new());
                            error_entries.push(entry);
                            // print error_entries as json
                            println!("error_entries: {:?}", error_entries);

                            // Prune old error entries
                            // Prune old error entries
                            if error_entries.len() > max_error_entries {
                                let prune_threshold = chrono::Local::now()
                                    - chrono::Duration::from_std(max_error_age).unwrap();
                                error_entries.retain(|&(_, ref timestamp)| {
                                    let timestamp = chrono::DateTime::parse_from_rfc3339(timestamp)
                                        .ok()
                                        .map(|timestamp| {
                                            timestamp.with_timezone(&chrono::FixedOffset::east(0))
                                        }) // Convert to DateTime<FixedOffset>
                                        .unwrap_or_else(|| {
                                            chrono::Local::now()
                                                .with_timezone(&chrono::FixedOffset::east(0))
                                        }); // Convert to DateTime<FixedOffset> with a default timezone
                                    timestamp >= prune_threshold
                                });
                            }
                        }
                    }
                }
                None => println!("no logs found"),
            }
        }

        // Sleep for a specified interval (e.g., 30 seconds) before checking again
        thread::sleep(Duration::from_secs(2));
    }
}
