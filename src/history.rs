use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

pub struct History {
    file_path: PathBuf,
}

impl History {
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let file_path = home_dir.join(".funsh_history");
        History { file_path }
    }

    pub fn load(&self) -> Vec<String> {
        if self.file_path.exists() {
            let file = OpenOptions::new().read(true).open(&self.file_path).unwrap();
            BufReader::new(file)
                .lines()
                .filter_map(|line| line.ok())
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn append(&self, command: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .unwrap();
        writeln!(file, "{}", command).unwrap();
    }
}
