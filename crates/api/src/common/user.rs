use db_schema::models::users::{User, UserAggregation, UserForm};
use db_schema::{naive_now, schema::users, UserId};

// use db_views::resource_view;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};
use futures::future::{ok, Ready};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
//
//
//
#[derive(Deserialize)]
pub struct UserApi {}
//
//
//
#[derive(Clone, Default, Deserialize, Debug, Serialize)]
pub struct UserRequest {
    pub fname: Option<String>,
    pub lname: Option<String>,
    pub gender: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub user_name: Option<String>,
    pub pwd: Option<String>,
}
//
//
//
