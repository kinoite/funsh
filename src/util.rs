use std::fs;
use std::path::{Path, PathBuf};

pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn expand_tilde(path: &str) -> PathBuf {
    if let Some(home) = dirs::home_dir() {
        if path.starts_with("~") {
            return PathBuf::from(path.replacen("~", &home.to_string_lossy(), 1));
        }
    }
    PathBuf::from(path)
}

pub fn read_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}
