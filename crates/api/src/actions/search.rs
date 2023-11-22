use crate::actions::ManagementTrait;
use crate::common::{
    connection, user::*, CollectionRequest, SearchApi, SearchRequest, SingularRequest,
};
use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    web::Json,
    Error, FromRequest, HttpRequest, HttpResponse, Responder,
};
use db_queries::models::users::users::User_;
use db_schema::models::model_error::{ApiError, ApiErrorEnum};
use db_schema::models::products::Product;
use db_schema::schema::{products, products::dsl::*};
use db_schema::{models::users::*, UserId};
use db_schema::{
    models::users::{User, UserForm},
    naive_now,
};
use db_views::users::users_view::UserView;
use db_views::RequestCollection;
use diesel::query_dsl::methods::FilterDsl;
use diesel::PgTextExpressionMethods;
use diesel::RunQueryDsl;
use futures::future::{ok, Ready};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::Value;
use utils::claims::{self, Claims};

#[async_trait::async_trait(?Send)]
impl ManagementTrait<SearchRequest> for SearchApi {
    type Response = HttpResponse;

    async fn create_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<SearchRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);
        println!("search value = {:#?}", _data.search_value.clone().unwrap());

        let key = _data.search_value.clone().unwrap();

        // Build the LIKE pattern for the search
        let pattern = format!("%{}%", key);

        // Perform the search query
        let result = products::table
            .filter(products::name.ilike(pattern))
            .load::<Product>(&_conn)
            .unwrap();

        if result.len() >= 1 {
            let _body = serde_json::to_string(&result)?;
            Ok(HttpResponse::Created()
                .content_type(ContentType::json())
                .body(_body))
        } else {
            // No data found
            let error_message = "Data not found";
            Ok(HttpResponse::NotFound()
                .content_type(ContentType::plaintext())
                .body(error_message))
        }
    }
}
