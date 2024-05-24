use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::hosts::{Host, Hosts};
use crate::database::models::{User, Group, Status};


/// Generic Response for basic HTTP reponses
#[derive(Serialize, Debug)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

/// Response for `systemctl` outputs
#[derive(Serialize, Debug)]
pub struct SystemctlResponse {
    pub status: String,
    pub count: usize,
    pub data: Vec<String>,
}

/// Response for `systemctl show` outputs
#[derive(Serialize, Debug)]
pub struct SystemctlShowResponse {
    pub status: String,
    pub count: usize,
    pub data: HashMap<String, String>,
}


/// Response for Host
/// `Host` Object
#[derive(Serialize, Debug)]
pub struct HostResponse {
    pub status: String,
    pub data: Host,
}

/// Response for multiple Hosts
/// NOTE: the `data` field does not contain an `AllHosts` object, but a 
/// `Hosts` Object
#[derive(Serialize, Debug)]
pub struct HostsResponse {
    pub status: String, 
    pub data: Hosts,
}

/// Response for User
#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: User,
}

/// Response for Users
#[derive(Serialize, Debug)]
pub struct UsersResponse {
    pub status: String,
    pub data: Vec<User>
}

/// Response for Group
/// `Group` object
#[derive(Serialize, Debug)]
pub struct GroupResponse {
    pub status: String,
    pub data: Group,
}

/// Response for Groups
/// `Vec<Group>` object
#[derive(Serialize, Debug)]
pub struct GroupsResponse {
    pub status: String,
    pub data: Vec<Group>,
}

/// Response for Service Status
/// `Status` object
#[derive(Serialize, Debug)]
pub struct StatusResponse {
    pub status: String,
    pub data: Status,
}

/// Response for Service Status
#[derive(Serialize, Debug)]
pub struct StatusesResponse {
    pub status: String,
    pub data: Vec<Status>,
}


// NEW RECORDS FROM HTML REQUEST
/******************************************************************************/
use diesel::prelude::*;
use crate::schema::users;
use chrono::prelude::*;

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = users)]
pub struct UserRequest {
    pub name: String,
    pub password: String,
    pub group_accts_id: Option<i64>,
    pub active: Option<bool>,
}

use crate::schema::group_accts;

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = group_accts)]
pub struct GroupRequest {
    pub parent_id: Option<i64>,
    pub name: String,
}

use crate::schema::service_status;

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = service_status)]
pub struct StatusRequest {
    pub hostname: String,
    pub name: String,
    pub description: Option<String>,
    pub enabled: Option<bool>,
    pub active_status: Option<String>,
    pub last_check: Option<NaiveDateTime>,
}

