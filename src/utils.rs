use crate::global::MONTY_DB_PATH;
use std::process::{Command, Stdio};
use std::io;
use std::time::Duration;
use std::thread;

pub(crate) fn start_subprocess() {
    let mut cmd = Command::new(MONTY_DB_PATH);
    println!("{:?}", cmd);

    let child: Result<std::process::Child, io::Error> = cmd.stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn();

    match child {
        Ok(mut child) => {
            thread::sleep(Duration::from_millis(500));

            match child.try_wait() {
                Ok(Some(_)) => {
                    eprintln!("Failure: setup superowner credentials first.");
                }
                Ok(None) => {
                    println!("{}", "Engine started");
                }
                Err(e) => {
                    eprintln!("Error status: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to start engine: {}", e);
        }
    }
}

pub(crate) fn stop_subprocess() {
    if cfg!(target_os = "windows") {
        stop_subprocess_windows();
    } else {
        #[cfg(unix)]
        stop_subprocess_unix();
    }
}

#[cfg(target_os = "windows")]
fn stop_subprocess_windows() {
    let output = Command::new("tasklist")
        .arg("/FI")
        .arg(format!("IMAGENAME eq {}", "monty_db.exe"))
        .output()
        .expect("failed to execute tasklist");

    let output_str = String::from_utf8(output.stdout).expect("invalid UTF-8");
    let lines: Vec<&str> = output_str.split('\n').collect();

    if lines.len() <= 1 {
        println!("Engine has not been running.");
        return;
    }

    let pid_line: Option<&&str> = lines.iter().skip(1).find(|line| line.contains("monty_db.exe"));

    if let Some(line) = pid_line {
        let pid: Option<u32> = line.split_whitespace()
            .nth(1)
            .and_then(|pid_str| pid_str.parse().ok());

        if let Some(pid) = pid {
            let result: Result<std::process::Output, io::Error> = Command::new("taskkill")
                .arg("/PID")
                .arg(pid.to_string())
                .arg("/F")
                .output();

            match result {
                Ok(_) => println!("Engine stopped"),
                Err(err) => eprintln!("Failed to stop engine: {}", err),
            }

        } else {
            eprintln!("Failed to parse PID from line: {}", line);
        }
    } else {
        println!("No process with the name found.");
    }
}

#[cfg(unix)]
fn stop_subprocess_unix() {
    let output = Command::new("pgrep")
        .arg("-f")
        .arg(MONTY_DB_PATH)
        .output()
        .expect("failed to execute pgrep");

    let pid_str = String::from_utf8(output.stdout)
        .expect("invalid UTF-8")
        .trim()
        .to_string();

    if pid_str.is_empty() {
        println!("Engine has not been running.");
        return;
    }

    let pid = match pid_str.parse::<u32>() {
        Ok(pid) => pid,
        Err(err) => {
            eprintln!("Failed to parse PID: {}. {}", err, "Please kill the process manually.");
            return;
        }
    };

    match nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid as i32), nix::sys::signal::Signal::SIGTERM) {
        Ok(_) => println!("{}", "Engine stopped"),
        Err(err) => eprintln!("Failed to stop engine: {}", err),
    }
}
