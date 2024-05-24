use std::error::Error;
use std::thread;
use chrono::Utc;
use crate::database::crudops;
use crate::hosts::Hosts;
use crate::service::show_service;
use crate::GenericError;

// TODO: Make async and call `show_service` via API
pub fn update_service_status(hostname: &str, service: &str) -> Result<(), Box<dyn Error>> {
    let status = show_service(hostname, service)?;
    let enabled: Option<bool> = match &status.get("UnitFileState") {
        Some(state) => {
            if (&state[..]).eq("enabled") {
                Some(true)
            } else {
                Some(false)
            }
        }
        _ => Some(false),
    };

    let description: Option<&str> = match status.get("Description") {
        Some(descrip) => Some(descrip),
        None => None,
    };

    let status: Option<&str> = match status.get("ActiveState") {
        Some(state) => Some(state),
        None => None,
    };

    crudops::insert_status(
        hostname,                       // hostname: &'a str
        service,                        // name: &'a str
        description,                    // description: Option<&'a str>
        enabled,                        // enabled: Option<&'a str>
        status,                         // active_status: Option<&'a str>
        Some(Utc::now().naive_utc()),   // last_check: Option<NaiveDateTime
    )?;
    Ok(())
}

/// Updates every service found within `hosts.json`
/// Spawns a thread for each `Host` as opposed to each service
pub fn update_host_json_services() -> Result<(), Box<dyn Error>> {
    let hosts = match Hosts::init_from_file() {
        Ok(hosts) => hosts,
        Err(_) => {
            return Err(Box::new(GenericError("error".to_string())));
        }
    };
    let mut thread_handles = vec![];
    for host in hosts.get_hosts() {
        let handle = thread::spawn(move || {
            for svc in host.services {
                match update_service_status(&host.hostname, &svc) {
                    Ok(()) => {
                        println!("SUCCESS: updated {} on {}", host.hostname, svc);
                    }
                    Err(_err) => {
                        eprintln!("WARNING: did not update {} on {}", host.hostname, svc);
                    }
                }
            }
        });
        thread_handles.push(handle);
    }
    for handle in thread_handles {
        handle.join().unwrap();
    }
    Ok(())
}
