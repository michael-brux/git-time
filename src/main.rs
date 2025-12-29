use chrono::Local;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::{exit, Command};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // 1. Logging
    log_call(&args);

    // 2. Prepare the Privacy Date (midnight today)
    let privacy_date = Local::now().format("%Y-%m-%d 00:00:00").to_string();

    // 3. Find the real git executable
    // Avoid using "git-time" or "git-date" itself if they are in PATH to prevent recursion
    let git_path = which::which("git").unwrap_or_else(|_| PathBuf::from("/usr/bin/git"));

    // 4. Handle "commit" command
    // We check if "commit" is present as a standalone argument (not inside a string)
    let is_commit = args.iter().any(|arg| arg == "commit");

    let mut cmd = Command::new(git_path);
    cmd.args(&args);

    if is_commit {
        cmd.env("GIT_AUTHOR_DATE", &privacy_date)
           .env("GIT_COMMITTER_DATE", &privacy_date);
    }

    let status = cmd.status().expect("failed to execute git");
    exit(status.code().unwrap_or(1));
}

fn log_call(args: &[String]) {
    if let Some(mut home) = home::home_dir() {
        home.push(".log/git-time.log");

        // Ensure directory exists if you want, but for simplicity we just try to open
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&home)
        {
            let now = Local::now().format("%Y-%m-%d %H:%M:%S");
            let _ = writeln!(file, "{} | ARGS: {}", now, args.join(" "));
        }
    }
}