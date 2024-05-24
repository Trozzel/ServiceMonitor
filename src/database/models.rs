use chrono::prelude::*;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User{
    pub id: i64,
    pub name: String,
    pub password: String,
    pub group_accts_id: Option<i64>,
    pub active: bool,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
}


#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = crate::schema::group_accts)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Group{
    pub id: i64,
    parent_id: Option<i64>,
    pub name: String,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
}

#[derive(Queryable, QueryableByName, Selectable, Serialize, Insertable, Debug)]
#[diesel(table_name = crate::schema::service_status)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Status{
    pub id: i64,
    pub hostname: String,
    pub name: String,
    pub description: Option<String>,
    pub enabled: Option<bool>,
    pub active_status: Option<String>,
    pub last_check: Option<NaiveDateTime>
}


// NEW RECORDS
/******************************************************************************/
use crate::schema::users;

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub password: &'a str,
    pub group_accts_id: Option<i64>,
    pub active: Option<bool>,
}

use crate::schema::group_accts;

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = group_accts)]
pub struct NewGroup<'a> {
    pub parent_id: Option<i64>,
    pub name: &'a str,
}

use crate::schema::service_status;

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = service_status)]
pub struct NewStatus<'a> {
    pub hostname: &'a str,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub enabled: Option<bool>,
    pub active_status: Option<&'a str>,
    pub last_check: Option<NaiveDateTime>,
}

