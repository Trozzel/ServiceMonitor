use crate::database::crudops::{get_json_statuses, get_status_from_db, insert_status};
use crate::response::{GenericResponse, StatusRequest, StatusResponse, StatusesResponse};
use diesel::QueryResult;
use rocket::{get, http::Status, put, response::status::Custom, serde::json::Json};

/// Insert `Status` by object
pub fn insert_status_by_obj(status: StatusRequest) -> QueryResult<usize> {
    let mut description_str = String::new();
    let description: Option<&str> = match status.description {
        Some(s) => {
            description_str = s.clone();
            Some(&description_str)
        }
        None => {
            drop(description_str);
            None
        }
    };

    let mut active_status_str = String::new();
    let active_status: Option<&str> = match status.active_status {
        Some(s) => {
            active_status_str = s.clone();
            Some(&active_status_str)
        }
        None => {
            drop(active_status_str);
            None
        }
    };

    insert_status(
        &status.hostname,
        &status.name,
        description,
        status.enabled,
        active_status,
        status.last_check,
    )
}

/// Update the `status_service` table with latest service status
#[put("/svc/update_status", data = "<status>")]
pub async fn update_status_api(
    status: Json<StatusRequest>,
) -> Result<Custom<Json<GenericResponse>>, Custom<Json<GenericResponse>>> {
    let new_status = status.0;
    let servicename = new_status.name.clone();
    let hostname = new_status.hostname.clone();

    match insert_status_by_obj(new_status) {
        Ok(_sz) => {
            let response = GenericResponse {
                status: String::from("success"),
                message: format!("'{}' on {} successfully updated.", servicename, hostname),
            };
            return Ok(Custom(Status::Created, Json(response)));
        }
        Err(err) => {
            let error_response = GenericResponse {
                status: String::from("fail"),
                message: format!("{:?}", err),
            };
            return Err(Custom(Status::InternalServerError, Json(error_response)));
        }
    }
}

/// Get `status_service` by `servicename` and `host`
#[get("/svc/get_latest_status?<host>&<service>")]
pub fn get_latest_status_api(
    host: &str,
    service: &str,
) -> Result<Custom<Json<StatusResponse>>, Custom<Json<GenericResponse>>> {
    match get_status_from_db(host, service) {
        Ok(status) => Ok(Custom(
            Status::Ok,
            Json(StatusResponse {
                status: String::from("success"),
                data: status,
            }),
        )),
        Err(err) => {
            let response = GenericResponse {
                status: String::from("success"),
                message: format!("{:?}", err),
            };
            Err(Custom(Status::InternalServerError, Json(response)))
        }
    }
}

/// Get latetest statuses for every service listed in `host.json`
#[get("/svc/get_latest_statuses")]
pub fn get_latest_statuses_api() -> Result<Custom<Json<StatusesResponse>>, Custom<Json<GenericResponse>>> {
    match get_json_statuses() {
        Ok(statuses) => Ok(Custom(
            Status::Ok,
            Json(StatusesResponse {
                status: String::from("success"),
                data: statuses,
            }),
        )),
        Err(err) => {
            let response = GenericResponse {
                status: String::from("success"),
                message: format!("{:?}", err),
            };
            Err(Custom(Status::InternalServerError, Json(response)))
        }
    }
}
