use crate::{
    schema::{user_aggregations, users},
    UserAggregationId, UserId,
};

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: UserId,
    pub fname: String,
    pub lname: String,
    pub gender: String,
    pub email: String,
    pub phone_number: String,
    pub address: Option<String>,
    pub type_: String,
    pub user_name: String,
    pub pwd: String,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Clone, Default, Debug)]
#[table_name = "users"]
pub struct UserForm {
    pub fname: Option<String>,
    pub lname: Option<String>,
    pub gender: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub user_name: Option<String>,
    pub pwd: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Clone, Queryable, Identifiable, Associations, PartialEq, Debug, Serialize)]
#[belongs_to(User)]
#[table_name = "user_aggregations"]
pub struct UserAggregation {
    pub id: UserAggregationId,
    pub user_id: UserId,
    pub orders: i64,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Queryable,
    Identifiable,
    Insertable,
    Associations,
)]
#[table_name = "users"]
pub struct UserData {
    pub id: UserId,
    pub fname: String,
    pub lname: String,
    pub gender: String,
    pub email: String,
    pub address: String,
    pub type_: String,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}
