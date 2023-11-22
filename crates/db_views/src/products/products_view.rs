use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::ResponceCollection;
use db_queries::{models::products::products::Product_, ManagementAsyncTrait, ViewToVec};
use db_schema::CategoryId;
use db_schema::models::model_error::ModelError;
use db_schema::schema::categories;
use db_schema::schema::products::dsl::*;
use db_schema::schema::products::id;
use db_schema::{
    models::products::{Product, ProductForm},
    ProductId,
};
use diesel::PgConnection;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize, Clone)]
pub struct ProductView {
    #[serde(flatten)]
    pub item: Value,
}

type ProductViewTuple = (Product);

impl ViewToVec for ProductView {
    type DbTuple = ProductViewTuple;
    fn from_tuple_to_vec(items: Vec<Self::DbTuple>) -> Vec<Self> {
        items
            .iter()
            .map(|a| Self {
                item: json!(a.to_owned()),
            })
            .collect::<Vec<Self>>()
    }
}

impl ProductView {
    pub async fn create_item(
        _conn: &PgConnection,
        _form: &ProductForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<ProductView, ModelError> {
        let _item = Product::create_item(_conn, _form, &_fields, _expand).await?;

        let _return_item = _item.collect_fields(&_fields).await?;
        let vec = vec![_item];

        Ok(Self { item: _return_item })
    }

    pub async fn update_item(
        _conn: &PgConnection,
        _id: &ProductId,
        _form: &ProductForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError> {
        let item = Product::update_item(_conn, _id, _form, &_fields, _expand).await?;

        let _return_item = item.collect_fields(&_fields).await?;
        let vec = vec![item];
        Ok(Self { item: _return_item })
    }

    pub async fn delete_item(_conn: &PgConnection, _id: &ProductId) -> Result<usize, ModelError> {
        Product::delete_item(_conn, _id).await
    }

    pub async fn get_item(
        _conn: &PgConnection,
        _id: &ProductId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError> {
        let _item = Product::get_item(_conn, _id, &_fields, _expand).await?;

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

        let (items, _return_count, _has_more) = Product::get_collection(
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

        let mut _return_list: Vec<ProductView> = Vec::new();
        let mut i: usize = 0;
        for item in items.into_iter() {
            let _return_user = item.collect_fields(_fields).await?;
            _return_list.push(ProductView { item: _return_user });
            i = i + 1;
        }

        let mut _res = ResponceCollection::<ProductView> {
            count: Some(_return_list.len() as i64),
            total_counts: _return_count,
            has_more: _has_more,
            offset: *_offset,
            limit: *_limit,
            items: _return_list,
        };

        Ok(_res)
    }
    pub async fn decrease(
        _conn: &PgConnection,
        _id: &Vec<ProductId>,
    ) -> Result<String, ModelError> {
        for a in _id {
            let b = products
                .find(a)
                .select(quantity)
                .first::<i64>(_conn)
                .unwrap();

            let c = b - 1;

            diesel::update(products)
                .filter(id.eq(a))
                .set(quantity.eq(c))
                .execute(_conn);
        }

        Ok(String::new())
    }
    pub async fn image(
        _conn: &PgConnection,
        _id: &ProductId,
        _image: String,
    ) -> Result<String, ModelError> {
        diesel::update(products)
            .filter(id.eq(_id))
            .set(image.eq(_image))
            .execute(_conn);
        Ok(String::new())
    }
    pub async fn image1(
        _conn: &PgConnection,
        _id: &CategoryId,
        _image: String,
    ) -> Result<String, ModelError> {
        use db_schema::schema::categories::dsl::*;
        diesel::update(categories)
            .filter(id.eq(_id))
            .set(image.eq(_image))
            .execute(_conn);
        Ok(String::new())
    }
}
