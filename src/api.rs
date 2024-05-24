// EXPORT API HANDLER MODULES
/******************************************************************************/
pub mod service_handler;
pub mod host_handler;
pub mod db_handler;
pub mod helpers;


// BEGIN API MODULE
/******************************************************************************/
use rocket::{get, http::Status, serde::json::Json};
use crate::response::GenericResponse;

/// Is server alive
#[get("/isalive")]
pub async fn isalive_api() -> Result<Json<GenericResponse>, Status> {
    const MESSAGE: &str = "alive";

    let response_json = GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(Json(response_json))
}


