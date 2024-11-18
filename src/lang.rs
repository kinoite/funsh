use std::fs;

pub fn execute_script(file_path: &str) -> Result<(), String> {
    let content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    for line in content.lines() {
        crate::cmd::execute(line)?;
    }
    Ok(())
}
