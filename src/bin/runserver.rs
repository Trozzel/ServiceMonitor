use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

use svcmon::api::host_handler::{get_all_hosts_api, update_host_api, 
    remove_host_api, add_service_api, remove_service_api};
use svcmon::api::isalive_api;
use svcmon::api::service_handler::{restart_service_api, service_status_api,
    unit_files_api, start_service_api, stop_service_api, show_service_api};
use svcmon::api::db_handler::{update_status_api, get_latest_status_api,
    get_latest_statuses_api};
use svcmon::site::page_handler::{serve_files, serve_home_page,
    serve_index_page};

#[macro_use]
extern crate rocket;
extern crate serde;


#[launch]
fn rocket() -> _ {
    use rocket::http::Method;

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Put, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allowed_headers(AllowedHeaders::all())
        .allow_credentials(true);

    rocket::build()
        .configure(
            rocket::Config::figment()
                .merge(("address", "0.0.0.0"))
                .merge(("port", 8888)),
        )
        .manage(cors)
        .mount("/api", routes![isalive_api,])
        .mount("/api", routes![start_service_api,])
        .mount("/api", routes![stop_service_api,])
        .mount("/api", routes![restart_service_api,])
        .mount("/api", routes![service_status_api,])
        .mount("/api", routes![unit_files_api,])
        .mount("/api", routes![get_all_hosts_api,])
        .mount("/api", routes![update_host_api,])
        .mount("/api", routes![remove_host_api,])
        .mount("/api", routes![add_service_api,])
        .mount("/api", routes![remove_service_api,])
        .mount("/api", routes![show_service_api,])
        .mount("/api", routes![update_status_api,])
        .mount("/api", routes![get_latest_status_api,])
        .mount("/api", routes![get_latest_statuses_api,])
        .mount("/", routes![serve_home_page,])
        .mount("/", routes![serve_index_page, serve_files])
}
