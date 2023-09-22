#[derive(Debug, Clone)]
pub struct ErrorHistory {
    pub errors: Vec<(usize, String)>,
}

impl ErrorHistory {
    pub fn new() -> ErrorHistory {
        ErrorHistory {
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: &str) {
        let current_time = chrono::Local::now().to_string();
        let entry = (error.len(), current_time);
        self.errors.push(entry);
    }

    pub fn get_error_count(&self) -> usize {
        self.errors.len()
    }

    pub fn get_error_count_by_last_n_seconds(&self, n: usize) -> usize {
        let current_time = chrono::Local::now();
        let mut count = 0;
        for error in &self.errors {
            let error_time = chrono::DateTime::parse_from_rfc3339(&error.1).unwrap();
            let duration = current_time.signed_duration_since(error_time);
            if duration.num_seconds() < n as i64 {
                count += 1;
            }
        }
        count
    }

}
