use crate::config::read_config_file;

use serde::{Deserialize, Serialize};
use core::fmt;
use std::error::Error;
use std::fs::File;


// GENERIC ERROR
/******************************************************************************/
/// Generic error to satisfy Box<dyn Err>
#[derive(Debug)]
struct HostError (String);

impl fmt::Display for HostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Host error: {}", self.0)
    }
}

impl Error for HostError {}

// HOSTS STRUCTS
/******************************************************************************/
/// Host object representing each host to be monitored from `hosts.json`
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Host {
    pub hostname: String,
    pub services: Vec<String>,
}

// IMPL `Host`
//------------------------------------------------------------------------------
impl Host {
    /// Add a service to `services` Only called by `Hosts` object
    pub fn add_service(&mut self, service: &str) {
        self.services.push(service.to_string());
    }
    
    pub fn remove_service(&mut self, service: &str) {
        self.services.remove(
            self.services.iter()
            .position(|svc| svc == service)
            .unwrap()
        );
    }
}

/// Object representing each hosting environment: dev, stage, prod from
/// `hosts.json`
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Hosts {
    hosts: Vec<Host>,
}

// HOSTS METHODS
/******************************************************************************/
impl Hosts {
    /// Initialize `Hosts` from file
    /// Reads hosts from 'hosts.json'
    pub fn init_from_file() -> Result<Hosts, Box<dyn Error>> {
        let config_file = &read_config_file().hostsfile;

        let file = File::open(&config_file)?;

        let hosts: Hosts = serde_json::from_reader(file)?;

        Ok(hosts)
    }
    
    pub fn get_hosts(self) -> Vec<Host> {
        self.hosts
    }

    /// Add a `Host` to `Hosts`
    pub fn add_host(&mut self, host: Host) {
        self.hosts.push(host);
    }

    /// Remove a `Host` from `Hosts`
    pub fn remove_host(&mut self, host: &Host) -> Option<Host> {
        match self.hosts.iter().position(|h| h == host) {
            Some(pos) => Some(self.hosts.remove(pos)),
            None => None,
        }
    }

    /// Return a `&mut Host` by hostname
    pub fn get_host_by_hostname(&mut self, hostname: &str) -> Option<&mut Host> {
        match self.hosts.iter_mut()
            .position(|host| host.hostname == hostname) {
                Some(pos) => Some(&mut self.hosts[pos]),
                None => None,
            }
    }

    /// Remove `Host` by hostname
    pub fn remove_host_by_hostname(&mut self, hostname: &str) -> Option<Host>{
        match self.hosts.iter_mut()
            .position(|host| host.hostname == hostname) {
                Some(pos) => {
                    return Some(self.hosts.remove(pos));
                }
                None => None,
            }
    }

    /// Add service to a `Host`
    pub fn add_service(&mut self, hostname: &str, service: &str) -> Result<(), Box<dyn Error>> {
        match self.get_host_by_hostname(hostname) {
            Some(host) => {
                host.services.push(service.to_string());
            }
            None => { 
                let err_msg = format!("Could not find host by hostname, '{}'.", hostname);
                return Err(Box::new(HostError(err_msg)));
            }
        };

        Ok(())
    }

    /// Remove service from `Host`
    pub fn remove_service(&mut self, hostname: &str, service: &str) -> Result<(), Box<dyn Error>> {
        match self.get_host_by_hostname(hostname) {
            Some(host) => {
                match host.services.iter_mut().position(|svc| svc == service) {
                    Some(pos) => {
                        host.services.remove(pos);
                    },
                    None => {
                        let err_str = format!("Service, '{}', does not exists for host, '{}'.", service, hostname);
                        return Err(Box::new(HostError(err_str)));
                    },
                };
            },
            None => {
                let err_msg = format!("Could not find host by hostname, '{}'.", hostname);
                return Err(Box::new(HostError(err_msg)));
            }
        };
        Ok(())
    }

    pub fn write_to_file(self) -> Result<(), Box<dyn Error>> {
        let config_file = &read_config_file().hostsfile;
        let file = File::create(&config_file)?;

        serde_json::to_writer_pretty(file, &self)?;

        Ok(())
    }
}

/// Add `Host` to `hosts.json`
pub fn add_host_to_hosts_file(host: Host) -> Result<(), Box<dyn Error>> {
    let mut hosts = Hosts::init_from_file()?;
    hosts.remove_host(&host);
    hosts.add_host(host);
    hosts.write_to_file()?;

    Ok(())
}

/// Add a service to a `Host` and add to the `hosts.json` file
pub fn add_svc_to_hosts_file(hostname: &str, service: &str) -> Result<(), Box<dyn Error>> {
    let mut hosts = Hosts::init_from_file()?;
    let host: &mut Host = match hosts.get_host_by_hostname(hostname) {
        Some(host) => host,
        None => {
            return Err(Box::new(HostError("No host found by that hostname".into())));
        }
    };

    host.add_service(service);
    hosts.write_to_file()?;

    // Rewrite json file
    Ok(())
}

/// Remove a service from a `Host` and write to 'hosts.json' file
pub fn rem_svc_from_hosts_file(hostname: &str, service: &str) -> Result<(), Box<dyn Error>> {
    let mut hosts = Hosts::init_from_file()?;
    hosts.remove_service(hostname, service)?;
    hosts.write_to_file()?;
    Ok(())
}
