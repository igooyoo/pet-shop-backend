use crate::{
    schema::{order_items, user_orders},
    OrderId, OrderItemId, ProductId, UserId,
};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
#[table_name = "user_orders"]
pub struct Order {
    pub id: OrderId,
    pub user_id: UserId,
    pub order_date: chrono::NaiveDateTime,
    pub order_status: String,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Clone, Default, Debug)]
#[table_name = "user_orders"]
pub struct OrderForm {
    pub user_id: Option<UserId>,
    pub order_status: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}
