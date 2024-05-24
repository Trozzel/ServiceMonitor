use chrono::prelude::*;
use diesel::{mysql::MysqlConnection, sql_query};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::error::Error;
use std::thread;
use std::sync::mpsc;

use super::models;
use crate::hosts::Hosts;
use crate::schema::{self};
use crate::GenericError;

pub const TABLES: &[&str] = &["users", "group_accts", "service_status"];
pub const CRUD_IPS: &[&str] = &["insert", "update", "insert", "delete"];

// ESTABLISH CONNECTION TO THE DATABASE
/******************************************************************************/
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/*----------------------------------USERS-------------------------------------*/

// INSERT SINGLE USER
/******************************************************************************/
pub fn insert_user(
    conn: &mut MysqlConnection,
    name: &str,
    password: &str,
    group_accts_id: Option<i64>,
    active: Option<bool>,
) -> QueryResult<usize> {
    let new_user = models::NewUser {
        name,
        password,
        group_accts_id,
        active,
    };

    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(conn)
}

// RETRIEVE ALL USERS
/******************************************************************************/
pub fn get_all_users() -> Result<Vec<models::User>, Box<dyn Error>> {
    let conn = &mut establish_connection();
    let results: Vec<models::User> = schema::users::table
        .select(models::User::as_select())
        .load(conn)?;

    Ok(results)
}

// RETRIEVE USER BY NAME
/******************************************************************************/
pub fn get_user_by_username(username: &str) -> Result<models::User, Box<dyn Error>> {
    let conn = &mut establish_connection();
    let mut result: Vec<models::User> = schema::users::table
        .filter(schema::users::name.eq(username))
        .limit(1)
        .select(models::User::as_select())
        .load(conn)?;

    let result: models::User = result.pop().ok_or(format!(
        "Error retrieving '{}' from table, `users`",
        username
    ))?;

    Ok(result)
}

// GET USER BY ID
/******************************************************************************/
pub fn get_user_by_id(id: i64) -> Result<models::User, Box<dyn Error>> {
    let conn = &mut establish_connection();
    let mut result: Vec<models::User> = schema::users::table
        .filter(schema::users::id.eq(id))
        .limit(1)
        .select(models::User::as_select())
        .load(conn)?;

    let result: models::User = result.pop().ok_or(format!(
        "Error retrieving `User` by `id` = {} from  `users`",
        id
    ))?;

    Ok(result)
}

// ADD USER
/******************************************************************************/
pub fn add_user_to_users(
    name: &str,
    password: &str,
    group_accts_id: Option<i64>,
    active: Option<bool>,
) -> QueryResult<usize> {
    let conn = &mut establish_connection();

    let new_user = models::NewUser {
        name,
        password,
        group_accts_id,
        active,
    };

    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(conn)
}

/*----------------------------------GROUPS------------------------------------*/

// INSERT SINGLE GROUP
/******************************************************************************/
pub fn insert_group(
    conn: &mut MysqlConnection,
    parent_id: Option<i64>,
    name: &str,
) -> QueryResult<usize> {
    let new_group_acct = models::NewGroup { parent_id, name };

    diesel::insert_into(schema::group_accts::table)
        .values(&new_group_acct)
        .execute(conn)
}

// RETRIEVE ALL GROUPS
/******************************************************************************/
pub fn get_all_groups() -> Result<Vec<models::Group>, Box<dyn Error>> {
    let conn = &mut establish_connection();
    let results: Vec<models::Group> = schema::group_accts::table
        .select(models::Group::as_select())
        .load(conn)?;

    Ok(results)
}

// RETRIEVE GROUP BY NAME
/******************************************************************************/
pub fn get_group_by_name(groupname: &str) -> Result<models::Group, Box<dyn Error>> {
    let conn = &mut establish_connection();
    let mut result: Vec<models::Group> = schema::group_accts::table
        .filter(schema::group_accts::name.eq(groupname))
        .limit(1)
        .select(models::Group::as_select())
        .load(conn)?;

    let result: models::Group = result.pop().ok_or(format!(
        "Error retrieving '{}' from table, `group_accts`",
        groupname
    ))?;

    Ok(result)
}

// GET GROUP BY ID
/******************************************************************************/
pub fn get_group_by_id(id: i64) -> Result<models::Group, Box<dyn Error>> {
    let conn = &mut establish_connection();
    let mut result: Vec<models::Group> = schema::group_accts::table
        .filter(schema::group_accts::id.eq(id))
        .limit(1)
        .select(models::Group::as_select())
        .load(conn)?;

    let result: models::Group = result.pop().ok_or(format!(
        "Error retrieving `Group` by `id` = {} from  `group_accts`",
        id
    ))?;

    Ok(result)
}

// ADD GROUP
/******************************************************************************/
pub fn add_group_to_groups(parent_id: Option<i64>, name: &str) -> QueryResult<usize> {
    let conn = &mut establish_connection();

    let new_group = models::NewGroup { parent_id, name };

    diesel::insert_into(schema::group_accts::table)
        .values(&new_group)
        .execute(conn)
}

/*----------------------------------STATUS------------------------------------*/

// INSERT STATUS
/******************************************************************************/
pub fn insert_status(
    hostname: &str,
    name: &str,
    description: Option<&str>,
    enabled: Option<bool>,
    active_status: Option<&str>,
    last_check: Option<NaiveDateTime>,
) -> QueryResult<usize> {
    let conn = &mut establish_connection();

    // TODO: this should be DEFAULT CURRENT_TIMESTAMP in the SQL
    let last_check = match last_check {
        Some(datetime) => Some(datetime),
        None => Some(Utc::now().naive_utc()),
    };

    let new_status = models::NewStatus {
        hostname,
        name,
        description,
        enabled,
        active_status,
        last_check,
    };

    diesel::insert_into(schema::service_status::table)
        .values(&new_status)
        .execute(conn)
}

// RETRIEVE ALL STATUSS
/******************************************************************************/
pub fn get_all_statuses() -> Result<Vec<models::Status>, Box<dyn Error>> {
    let conn = &mut establish_connection();
    let results: Vec<models::Status> = schema::service_status::table
        .filter(schema::service_status::last_check.is_not_null())
        .select(models::Status::as_select())
        .load(conn)?;

    Ok(results)
}

// GET ALL LATEST STATUSES
/******************************************************************************/
/// Perform raw SQL as diesel does not implement true `GROUP BY`
pub fn get_latest_statuses() -> Result<Vec<models::Status>, Box<dyn Error>> {
    let conn = &mut establish_connection();

    const SQL_LATEST_STATUSES: &str = "SELECT * FROM (SELECT * FROM service_status WHERE last_check IS NOT NULL ORDER BY last_check DESC) AS temp GROUP BY temp.hostname, temp.name;";

    match sql_query(SQL_LATEST_STATUSES).load(conn) {
        Ok(statuses) => Ok(statuses),
        Err(err) => Err(Box::new(GenericError(format!("{:?}", err)))),
    }
}

/// Get latest statuses from `service_status` table for hosts and services 
/// within `host.json` file
pub fn get_json_statuses() -> Result<Vec<models::Status>, Box<dyn Error>> {
    let hosts = match Hosts::init_from_file() {
        Ok(hosts) => hosts,
        Err(_) => {
            return Err(Box::new(GenericError("error".to_string())));
        }
    };

    enum SendStatus {
        Status(models::Status),
        Error(String),
    }

    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    let mut hosts_from_db: Vec<models::Status> = vec![];
    for host in hosts.get_hosts() {
        let tx_cpy = mpsc::Sender::clone(&tx);
        // TODO: Use an async library to better handle errors from 
        // `get_status_from_db`
        let handle = thread::spawn(move || {
            for service in host.services {
                let tmp = match get_status_from_db(&host.hostname, &service)  {
                    Ok(status) => {
                        SendStatus::Status(status)
                    },
                    Err(e) => {
                        SendStatus::Error(format!("Error retrieving,{}, from {}: {}",
                                                  &host.hostname, service, e))
                    },
                };
                if tx_cpy.send(tmp).is_err() {
                    println!("Breaking on {}, {}.", &host.hostname, &service);
                    break;
                }
            }
        });
        handles.push(handle);
    }
    // Must manually `drop(tx)` since doesn't send anything
    drop(tx);

    // Load statuses from threads
    for status_res in rx {
        match status_res {
            SendStatus::Status(status) => {
                hosts_from_db.push(status);
            }
            SendStatus::Error(e_str) => eprintln!("{}", e_str),
        }
    }

    Ok(hosts_from_db)
}

// RETRIEVE STATUS BY NAME
/******************************************************************************/
pub fn get_status_from_db(
    hostname: &str,
    servicename: &str,
) -> Result<models::Status, Box<dyn Error>> {
    let conn = &mut establish_connection();
    let mut result: Vec<models::Status> = schema::service_status::table
        .filter(schema::service_status::hostname.eq(hostname))
        .filter(schema::service_status::name.eq(servicename))
        .order(schema::service_status::last_check.desc())
        .limit(1)
        .select(models::Status::as_select())
        .load(conn)?;

    let result: models::Status = result.pop().ok_or(format!(
        "Error retrieving service: '{}' on host: '{}', from table, `service_status`",
        servicename, hostname
    ))?;

    Ok(result)
}

// GET STATUS BY ID
/******************************************************************************/
pub fn get_status_by_id(id: i64) -> Result<models::Status, Box<dyn Error>> {
    let conn = &mut establish_connection();
    let mut result: Vec<models::Status> = schema::service_status::table
        .filter(schema::service_status::id.eq(id))
        .limit(1)
        .select(models::Status::as_select())
        .load(conn)?;

    let result: models::Status = result.pop().ok_or(format!(
        "Error retrieving `Status` by `id` = {} from  `service_status`",
        id
    ))?;

    Ok(result)
}
