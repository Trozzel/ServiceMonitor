use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::{io, process};


/// Convert Vec<u8> to Vec<String>
/// Since the return of `output()` is io::Result<Vec<u8>>, we convert this to
/// a vector of strings for nearly (if not every) `systemctl` call
/// TODO: 1. out_str = String::from_utf8(out); 2. out_vec = out_str.split("\n");
/// 3. keep all values according to specs below
fn vec_u8_to_vec_string(out: std::process::Output) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    lines.push(String::new());
    for elem in out.stdout.iter() {
        if *elem == 10 {
            lines.push(String::new());
            continue;
        }
        // Skip non-ASCII characters
        else if *elem > 127 {
            continue;
        }
        let line = lines
            .last_mut()
            .expect("Error getting last string in vector");
        line.push(char::from(*elem));
    }
    // Remove white spaces
    let mut lines: Vec<String> = lines.iter().map(|s| s.trim().to_string()).collect();
    lines.retain(|line| !line.is_empty());
    lines
}

#[derive(Debug, Serialize, Deserialize)]
struct SystemCtlStatus {
    status: String,
    stdout: String,
    stderr: String,
}

/// List unit files for given host
/// To retrieve only enabled unit-files, pass `enabled_only == true`
pub fn list_unit_files(host: &str, enabled_only: Option<bool>) -> io::Result<Vec<String>> {
    let out = process::Command::new("systemctl")
        .arg("-H")
        .arg(&host)
        .arg("list-unit-files")
        .output()?;

    let mut lines = vec_u8_to_vec_string(out);

    let enabled_only = enabled_only.unwrap_or(true);
    if enabled_only {
        lines = lines
            .into_iter()
            .filter(|line| line.contains("enabled"))
            .collect();
    }

    Ok(lines)
}

/// Get status of a given service on a given host
pub fn get_status<'a, 'b>(host: &'b str, service: &'a str) -> io::Result<Vec<String>> {
    let out = process::Command::new("systemctl")
        .arg("-H")
        .arg(&host)
        .arg("status")
        .arg(&service)
        .arg("-l")
        .output()?;

    let mut lines = vec_u8_to_vec_string(out);
    if !lines.is_empty() {
        lines[0] = format!("Description: {}", lines[0]);
    }

    Ok(lines)
}

/// See if service is active
pub fn active_status<'a, 'b>(host: &'a str, service: &'b str) -> Result<String, Box<dyn Error>> {
    let out = process::Command::new("systemctl")
        .arg("-H")
        .arg(&host)
        .arg("is-active")
        .arg(&service)
        .output()?;

    let mut stdout_str = String::new();
    for elem in out.stdout {
        stdout_str.push(char::from(elem));
    }
    stdout_str = stdout_str.trim().to_string();

    Ok(stdout_str)
}

/// See if service is enabled
pub fn enabled_status<'a, 'b>(host: &'a str, service: &'b str) -> Result<String, Box<dyn Error>> {
    let out = process::Command::new("systemctl")
        .arg("-H")
        .arg(&host)
        .arg("is-enabled")
        .arg(&service)
        .output()?;

    let mut stdout_str = String::new();
    for elem in out.stdout {
        stdout_str.push(char::from(elem));
    }
    stdout_str = stdout_str.trim().to_string();

    Ok(stdout_str)
}

/// Get full details of a service
pub fn show_service<'a, 'b>(
    host: &str,
    service: &str,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let out = process::Command::new("systemctl")
        .arg("-H")
        .arg(&host)
        .arg("show")
        .arg(&service)
        .output()?;

    let out = vec_u8_to_vec_string(out);

    let mut service_status: HashMap<String, String> = HashMap::new();
    for line in out {
        let key_val = line.split_once('=');
        if !key_val.is_some() {
            eprintln!(
                "WARNING: no '=' found in `systemctl -H {} show {}`: line == {}",
                host, service, line
            );
            continue;
        }
        let key_val = key_val.unwrap();

        service_status.insert(key_val.0.to_string(), key_val.1.to_string());
    }
    Ok(service_status)
}

/// Send command (`start`, `stop`, `restart`) to `systemctl`
/// NOTE: this function checks whether the command valid first
fn send_command<'a, 'b>(
    host: &'a str,
    service: &'b str,
    command: &'b str,
) -> io::Result<Vec<String>> {
    let out = process::Command::new("systemctl")
        .arg("-H")
        .arg(&host)
        .arg(&command)
        .arg(&service)
        .output()?;

    Ok(vec_u8_to_vec_string(out))
}

/// Restarts a service
/// Calls helper function, `send_command`
pub fn restart_service<'a, 'b>(host: &'a str, service: &'b str) -> io::Result<Vec<String>> {
    send_command(host, service, "restart")
}

/// Starts a service
/// Calls helper function, `send_command`
pub fn stop_service<'a, 'b>(host: &'a str, service: &'b str) -> io::Result<Vec<String>> {
    send_command(host, service, "stop")
}

/// Restarts a service
/// Calls helper function, `send_command`
pub fn start_service<'a, 'b>(host: &'a str, service: &'b str) -> io::Result<Vec<String>> {
    send_command(host, service, "start")
}
