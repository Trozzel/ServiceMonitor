use crate::response::{GenericResponse, SystemctlResponse, SystemctlShowResponse};
use crate::service::{get_status, list_unit_files, restart_service, show_service, start_service, stop_service};
use rocket::{get, http::Status, response::status::Custom, serde::json::Json};

/// `systemctl -H <host> -l status` API
#[get("/systemctl/status?<host>&<service>")]
pub async fn service_status_api(
    host: &str,
    service: &str,
) -> Result<Custom<Json<SystemctlResponse>>, Custom<Json<GenericResponse>>> {
    let status = get_status(&host, &service);
    match status {
        Ok(v) => {
            return Ok(Custom(
                Status::Ok,
                Json(SystemctlResponse {
                    status: String::from("success"),
                    count: v.len(),
                    data: v,
                }),
            ));
        }
        _ => {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Could not get the status from {}", host),
            };
            return Err(Custom(Status::InternalServerError, Json(error_response)));
        }
    }
}

/// `systemctl -H <host> show <service>`
#[get("/systemctl/show?<host>&<service>")]
pub async fn show_service_api(
    host: &str,
    service: &str
    ) -> Result<Json<SystemctlShowResponse>, Custom<Json<GenericResponse>>> {
    let sysctl_show = show_service(host, service);
    match sysctl_show {
        Ok(hashmap) => {
            let response = SystemctlShowResponse {
                status: String::from("success"),
                count: hashmap.len(),
                data: hashmap,
            };
            return Ok(Json(response));
        },
        Err(err) => {
            let response = GenericResponse {
                status: String::from("fail"),
                message: format!("{:?}", err),
            };
            return Err(Custom(Status::InternalServerError, Json(response)));
        }
    }
}

/// `systemctl start -H <host> <service>`
/// NOTE: A successful start does not mean that the service started
/// successfully. It means that the command was sent successfully.
/// - To check for successful start, call `service_status_api`
#[get("/systemctl/start?<host>&<service>")]
pub async fn start_service_api(
    host: &str,
    service: &str,
) -> Result<Custom<Json<SystemctlResponse>>, Custom<Json<GenericResponse>>> {
    let start = start_service(&host, &service);
    match start {
        Ok(vec) => {
            return Ok(Custom(
                Status::Ok,
                Json(SystemctlResponse {
                    status: String::from("success"),
                    count: vec.len(),
                    data: vec,
                }),
            ));
        }
        _ => {
            let error_response = GenericResponse {
                status: String::from("fail"),
                message: format!("Failed to start '{}' on '{}'", service, host),
            };
            return Err(Custom(Status::InternalServerError, Json(error_response)));
        }
    }
}

/// `systemctl stop -H <host> <service>`
/// NOTE: A successful stop does not mean that the service stoped
/// successfully. It means that the command was sent successfully.
/// - To check for successful stop, call `service_status_api`
#[get("/systemctl/stop?<host>&<service>")]
pub async fn stop_service_api(
    host: &str,
    service: &str,
) -> Result<Custom<Json<SystemctlResponse>>, Custom<Json<GenericResponse>>> {
    let stop = stop_service(&host, &service);
    match stop {
        Ok(vec) => {
            return Ok(Custom(
                Status::Ok,
                Json(SystemctlResponse {
                    status: String::from("success"),
                    count: vec.len(),
                    data: vec,
                }),
            ));
        }
        _ => {
            let error_response = GenericResponse {
                status: String::from("fail"),
                message: format!("Failed to stop '{}' on '{}'", service, host),
            };
            return Err(Custom(Status::InternalServerError, Json(error_response)));
        }
    }
}


/// `systemctl restart -H <host> <service>`
/// NOTE: A successful restart does not mean that the service restarted
/// successfully. It means that the command was sent successfully.
/// - To check for successful restart, call `service_status_api`
#[get("/systemctl/restart?<host>&<service>")]
pub async fn restart_service_api(
    host: &str,
    service: &str,
) -> Result<Custom<Json<SystemctlResponse>>, Custom<Json<GenericResponse>>> {
    let restart = restart_service(&host, &service);
    match restart {
        Ok(vec) => {
            return Ok(Custom(
                Status::Ok,
                Json(SystemctlResponse {
                    status: String::from("success"),
                    count: vec.len(),
                    data: vec,
                }),
            ));
        }
        _ => {
            let error_response = GenericResponse {
                status: String::from("fail"),
                message: format!("Failed to restart '{}' on '{}'", service, host),
            };
            return Err(Custom(Status::InternalServerError, Json(error_response)));
        }
    }
}

/// `systemctl -H <host> list-unit-files [grep enabled | awk '{print $1}']`
#[get("/systemctl/unit-files?<host>&<enabled_only>")]
pub async fn unit_files_api(
    host: &str,
    enabled_only: Option<bool>,
) -> Result<Custom<Json<SystemctlResponse>>, Custom<Json<GenericResponse>>> {
    match list_unit_files(&host, enabled_only) {
        Ok(vec) => {
            return Ok(Custom(
                Status::Ok,
                Json(SystemctlResponse {
                    status: String::from("success"),
                    count: vec.len(),
                    data: vec,
                }),
            ));
        }
        _ => {
            let error_response = GenericResponse {
                status: String::from("fail"),
                message: format!("Failed to get unit files on '{}'", host),
            };
            return Err(Custom(Status::InternalServerError, Json(error_response)));
        }
    }
}
