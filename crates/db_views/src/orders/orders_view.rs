use crate::diesel::ExpressionMethods;
use crate::diesel::GroupByDsl;
use crate::ResponceCollection;
use db_queries::{models::orders::orders::Order_, ManagementAsyncTrait, ViewToVec};
use db_schema::models::model_error::ModelError;
use db_schema::schema::{order_items, products};
use db_schema::{
    models::orders::{Order, OrderForm},
    schema::user_orders,
    OrderId,
};
use diesel::dsl::count;
use diesel::query_dsl::JoinOnDsl;
use diesel::sql_query;
use diesel::QueryDsl;
use diesel::{query_dsl, PgConnection, RunQueryDsl};
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize, Clone)]
pub struct OrderView {
    #[serde(flatten)]
    pub item: Value,
}

#[derive(Serialize, Clone, Default, Debug)]
pub struct ReportAllWithPrice {
    pub total_saled: i64,
    pub items: Vec<ReportAll>,
}

#[derive(Serialize, Clone, Default, Debug)]
pub struct ReportAll {
    pub name: String,
    pub quantity: i64,
}

type OrderViewTuple = (Order);

impl ViewToVec for OrderView {
    type DbTuple = OrderViewTuple;
    fn from_tuple_to_vec(items: Vec<Self::DbTuple>) -> Vec<Self> {
        items
            .iter()
            .map(|a| Self {
                item: json!(a.to_owned()),
            })
            .collect::<Vec<Self>>()
    }
}

impl OrderView {
    pub async fn create_item(
        _conn: &PgConnection,
        _form: &OrderForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<OrderView, ModelError> {
        let _item = Order::create_item(_conn, _form, &_fields, _expand).await?;
        println!("asdddd");
        let _return_item = _item.collect_fields(&_fields).await?;
        let vec = vec![_item];

        Ok(Self { item: _return_item })
    }

    pub async fn update_item(
        _conn: &PgConnection,
        _id: &OrderId,
        _form: &OrderForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError> {
        let item = Order::update_item(_conn, _id, _form, &_fields, _expand).await?;

        let _return_item = item.collect_fields(&_fields).await?;
        let vec = vec![item];
        Ok(Self { item: _return_item })
    }

    pub async fn delete_item(_conn: &PgConnection, _id: &OrderId) -> Result<usize, ModelError> {
        Order::delete_item(_conn, _id).await
    }

    pub async fn get_item(
        _conn: &PgConnection,
        _id: &OrderId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError> {
        let _item = Order::get_item(_conn, _id, &_fields, _expand).await?;

        let _return_item = _item.collect_fields(&_fields).await?;

        let vec = vec![_item];

        Ok(Self { item: _return_item })
    }

    pub async fn get_collection(
        _conn: &PgConnection,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
        _q: &Option<String>,
        _sort: &Option<Vec<String>>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _total_count: &Option<bool>,
    ) -> Result<ResponceCollection<Self>, ModelError> {
        let mut _return_count: Option<i64>;

        let (items, _return_count, _has_more) = Order::get_collection(
            _conn,
            _fields,
            _expand,
            _q,
            _sort,
            _offset,
            _limit,
            _total_count,
        )
        .await?;

        let mut _return_list: Vec<OrderView> = Vec::new();
        let mut i: usize = 0;
        for item in items.into_iter() {
            let _return_user = item.collect_fields(_fields).await?;
            _return_list.push(OrderView { item: _return_user });
            i = i + 1;
        }

        let mut _res = ResponceCollection::<OrderView> {
            count: Some(_return_list.len() as i64),
            total_counts: _return_count,
            has_more: _has_more,
            offset: *_offset,
            limit: *_limit,
            items: _return_list,
        };

        Ok(_res)
    }

    pub fn report_all(_conn: &PgConnection) -> Result<ReportAllWithPrice, ModelError> {
        let a = user_orders::table
            .select(user_orders::id)
            .filter(user_orders::order_status.eq("payed"))
            .load::<String>(_conn)
            .unwrap();

        let mut total_saled = 0;

        for i in a {
            let b = order_items::table
                .select(order_items::product_id)
                .filter(order_items::order_id.eq(i))
                .load::<String>(_conn)
                .unwrap();

            for j in b {
                let c = products::table
                    .select(products::price)
                    .find(j)
                    .first::<i64>(_conn)
                    .unwrap();
                total_saled = total_saled + c;
            }
        }

        let a = products::table
            .inner_join(order_items::table.on(products::id.eq(order_items::product_id)))
            .group_by(products::name)
            .select(products::name)
            .load::<String>(_conn)
            .unwrap();

        let b = products::table
            .inner_join(order_items::table.on(products::id.eq(order_items::product_id)))
            .group_by(products::name)
            .select(count(products::name))
            .load::<i64>(_conn)
            .unwrap();
        let mut report_all = Vec::new();
        let mut p = 0;
        for l in a {
            let report = ReportAll {
                name: l,
                quantity: b[p],
            };
            report_all.push(report);
            p += 1;
        }

        let bb = ReportAllWithPrice {
            total_saled: total_saled,
            items: report_all,
        };
        Ok(bb)
    }
}
