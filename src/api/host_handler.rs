use crate::api::helpers::request_types::HostnameWithService;
use crate::hosts::{add_host_to_hosts_file, rem_svc_from_hosts_file, Host, Hosts};
use crate::response::{GenericResponse, HostsResponse};

use rocket::{
    delete, get, http::Status, patch, post, put, response::status::Custom, serde::json::Json,
};

/// Get all hosts
/// Returns reference to the static object created that represents the
/// `hosts.json` file
#[get("/hosts/all")]
pub async fn get_all_hosts_api() -> Result<Custom<Json<HostsResponse>>, Status> {
    match Hosts::init_from_file() {
        Ok(hosts) => {
            let response = HostsResponse {
                status: String::from("success"),
                data: hosts,
            };
            return Ok(Custom(Status::Ok, Json(response)));
        }
        Err(_) => {
            return Err(Status::Conflict);
        }
    };
}

/// Add a host to 'hosts.json'
#[post("/hosts/add_host", data = "<host>")]
pub async fn add_host_api(
    host: Json<Host>,
) -> Result<Custom<Json<GenericResponse>>, Custom<Json<GenericResponse>>> {
    let hostname = host.0.hostname.clone();
    match add_host_to_hosts_file(host.0) {
        Ok(_) => {
            let msg = format!("Host, '{}', successfully added.", hostname);
            return Ok(Custom(
                Status::Ok,
                Json(GenericResponse {
                    status: String::from("success"),
                    message: msg,
                }),
            ));
        }
        Err(e) => {
            let err_msg = format!("Error adding host, '{}': {}", hostname, e);
            return Err(Custom(
                Status::InternalServerError,
                Json(GenericResponse {
                    status: String::from("fail"),
                    message: err_msg,
                }),
            ));
        }
    };
}

/// Add a service to `AllHosts`
#[put("/hosts/update_host", data = "<host>")]
pub async fn update_host_api(host: Json<Host>) -> Result<Custom<Json<GenericResponse>>, Status> {
    let mut hosts = match Hosts::init_from_file() {
        Ok(hosts) => hosts,
        Err(_) => {
            return Err(Status::Conflict);
        }
    };

    let new_host = host.0;
    let hostname = new_host.hostname.clone();
    hosts.remove_host(&new_host);
    match hosts.write_to_file() {
        Ok(()) => Ok(Custom(
            Status::Created,
            Json(GenericResponse {
                status: String::from("success"),
                message: format!("Host, '{}', successfully updated.", hostname),
            }),
        )),
        Err(_e) => Err(Status::Conflict),
    }
}

/// Remove service from `Host`
#[delete("/hosts/remove_host", data = "<hostname>")]
pub async fn remove_host_api(
    hostname: &str,
) -> Result<Custom<Json<GenericResponse>>, Custom<Json<GenericResponse>>> {
    match Hosts::init_from_file() {
        Ok(mut hosts) => match hosts.remove_host_by_hostname(hostname) {
            Some(_host) => {
                let success_response = GenericResponse {
                    status: String::from("sucess"),
                    message: format!("Host, '{}', successfully removed", hostname),
                };
                return Ok(Custom(Status::Created, Json(success_response)));
            }
            None => {
                let success_response = GenericResponse {
                    status: String::from("sucess"),
                    message: format!("Host, '{}', does not exist", hostname),
                };
                return Ok(Custom(Status::Ok, Json(success_response)));
            }
        },
        Err(e) => {
            let error_response = GenericResponse {
                status: String::from("failure"),
                message: format!("Interal error loading `hosts.json` file: {:?}", e),
            };
            return Err(Custom(Status::InternalServerError, Json(error_response)));
        }
    }
}

/// Add a service to a `Host`
#[post("/hosts/add_service", data = "<hostname_w_svc>")]
pub async fn add_service_api(
    hostname_w_svc: Json<HostnameWithService>,
) -> Result<Custom<Json<GenericResponse>>, Custom<Json<GenericResponse>>> {
    let hostname = hostname_w_svc.hostname.clone();
    let service = hostname_w_svc.service.clone();
    match rem_svc_from_hosts_file(&hostname, &service) {
        Ok(_) => {
            let msg = format!(
                "Service, '{}', successfully added to host, '{}'",
                hostname_w_svc.hostname, hostname_w_svc.service
            );
            return Ok(Custom(
                Status::Ok,
                Json(GenericResponse {
                    status: String::from("success"),
                    message: msg,
                }),
            ));
        }
        Err(e) => {
            let err_msg = format!(
                "Did not remove service, '{}', from host, '{}': {:?}",
                hostname_w_svc.hostname, hostname_w_svc.service, e
            );
            return Err(Custom(
                Status::Conflict,
                Json(GenericResponse {
                    status: String::from("fail"),
                    message: err_msg,
                }),
            ));
        }
    };
}

/// Remove a service from a `host`
#[patch("/hosts/remove_service", data = "<hostname_w_svc>")]
pub async fn remove_service_api(
    hostname_w_svc: Json<HostnameWithService>,
) -> Result<Custom<Json<GenericResponse>>, Custom<Json<GenericResponse>>> {
    let hostname = hostname_w_svc.hostname.to_owned();
    let service = hostname_w_svc.service.to_owned();
    match rem_svc_from_hosts_file(&hostname, &service) {
        Ok(_) => {
            let msg = format!(
                "Service, '{}', successfully removed from host, '{}'",
                hostname_w_svc.hostname, hostname_w_svc.service
            );
            return Ok(Custom(
                Status::Ok,
                Json(GenericResponse {
                    status: String::from("success"),
                    message: msg,
                }),
            ));
        }
        Err(e) => {
            let err_msg = format!(
                "Did not remove service, '{}', from host, '{}': {:?}",
                hostname_w_svc.hostname, hostname_w_svc.service, e
            );
            return Err(Custom(
                Status::Conflict,
                Json(GenericResponse {
                    status: String::from("fail"),
                    message: err_msg,
                }),
            ));
        }
    };
}
