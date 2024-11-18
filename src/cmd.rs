use std::env;
use std::fs;
use std::process::Command;

pub fn execute(input: &str) -> Result<(), String> {
    let args: Vec<&str> = input.split_whitespace().collect();

    match args.get(0) {
        Some(&"cd") => change_dir(args.get(1).unwrap_or(&"~")),
        Some(&"pwd") => print_working_dir(),
        Some(&"ls") => list_files(args.get(1).unwrap_or(&".")),
        Some(&"echo") => echo(args[1..].join(" ")),
        Some(&"fortune") => Ok(crate::fun::fortune()),
        _ => execute_external(input),
    }
}

fn change_dir(path: &str) -> Result<(), String> {
    let expanded_path = if path == "~" {
        env::var("HOME").unwrap_or_else(|_| "/".to_string())
    } else {
        path.to_string()
    };

    env::set_current_dir(expanded_path).map_err(|e| format!("cd: {}", e))
}

fn print_working_dir() -> Result<(), String> {
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
            Ok(())
        }
        Err(e) => Err(format!("pwd: {}", e)),
    }
}

fn list_files(path: &str) -> Result<(), String> {
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("{}", entry.file_name().to_string_lossy());
                }
            }
            Ok(())
        }
        Err(e) => Err(format!("ls: {}", e)),
    }
}

fn echo(input: String) -> Result<(), String> {
    println!("{}", input);
    Ok(())
}

fn execute_external(input: &str) -> Result<(), String> {
    let mut parts = input.split_whitespace();
    if let Some(command) = parts.next() {
        let args: Vec<&str> = parts.collect();
        let status = Command::new(command)
            .args(&args)
            .status()
            .map_err(|e| format!("funsh: {}: {}", command, e))?;

        if status.success() {
            Ok(())
        } else {
            Err(format!(
                "funsh: {} exited with status {}",
                command,
                status.code().unwrap_or(-1)
            ))
        }
    } else {
        Err("funsh: no command provided".to_string())
    }
}
