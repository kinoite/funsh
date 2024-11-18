use whoami::fallible;

pub fn render_ps1() -> String {
    let user = whoami::username();
    let host = fallible::hostname().unwrap_or_else(|_| "unknown".to_string());
    let cwd = std::env::current_dir()
        .unwrap_or_else(|_| "/".into())
        .display()
        .to_string();
    let home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
    let display_cwd = cwd.replace(&home, "~");

    format!(
        "{}@{} {} $ ",
        crate::coloransi::green(&user),
        crate::coloransi::blue(&host),
        display_cwd
    )
}

