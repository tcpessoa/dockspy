use std::process::Command;
use std::str;

use crate::error_history::ErrorHistory;

pub struct Container {
    pub id: String,
    pub name: String,
    pub logs: Option<String>,
    pub error_history: ErrorHistory,
}
impl Container {
    pub fn new(id: String, logs: Option<String>, name: String) -> Container {
        match logs {
            Some(logs) => Container {
                id,
                logs: Some(logs),
                name,
                error_history: ErrorHistory::new(),
            },
            None => Container {
                id,
                logs: None,
                name,
                error_history: ErrorHistory::new(),
            },
        }
    }

    pub fn with_logs(&self, logs: String) -> Container {
        Container {
            id: self.id.clone(),
            logs: Some(logs),
            name: self.name.clone(),
            error_history: self.error_history.clone(),
        }
    }

    pub fn get_logs_count(&self) -> usize {
        match self.logs {
            Some(ref logs) => logs.lines().count(),
            None => 0,
        }
    }
}

impl std::fmt::Debug for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Container {{ id: {}, name: {} }}", self.id, self.name)
    }
}

pub fn analyze_logs(log_data: &str) -> Vec<&str> {
    let mut errors: Vec<&str> = Vec::new();

    let lines = log_data.lines();

    for line in lines {
        if line.to_lowercase().contains("error") {
            errors.push(line);
        }
    }
    errors
}

pub fn get_container_logs(container_id: &str) -> String {
    let logs = Command::new("docker")
        .arg("logs")
        .arg(container_id)
        .output()
        .expect("failed to retrieve docker container logs");
    str::from_utf8(&logs.stdout).unwrap().to_string()
}

pub fn parse_container_info(info_to_parse: &str) -> Vec<Container> {
    let mut current_containers: Vec<Container> = Vec::new();

    for container_info in info_to_parse.lines() {
        let container_info: Vec<&str> = container_info.split_whitespace().collect();
        let container_id = container_info[0];
        let container_name = container_info[1];
        let container = Container::new(container_id.to_string(), None, container_name.to_string());
        let log_data = get_container_logs(container_id);

        current_containers.push(container.with_logs(log_data));
    }
    current_containers
}
