use crate::actions::ManagementTrait;
use crate::common::{
    connection, user::*, CollectionRequest, ReportAllApi, ReportAllRequest, SingularRequest,
};
use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    web::Json,
    Error, FromRequest, HttpRequest, HttpResponse, Responder,
};
use db_queries::models::users::users::User_;
use db_queries::ManagementAsyncTrait;
use db_schema::models::model_error::{ApiError, ApiErrorEnum};
use db_schema::models::orders::Order;
use db_schema::{models::users::*, UserId};
use db_schema::{
    models::users::{User, UserForm},
    naive_now,
};
use db_views::orders::orders_view::OrderView;
use db_views::users::users_view::UserView;
use db_views::RequestCollection;
use futures::future::{ok, Ready};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::Value;

#[async_trait::async_trait(?Send)]
impl ManagementTrait<ReportAllRequest> for ReportAllApi {
    type Response = HttpResponse;

    async fn get_item(
        _req: HttpRequest,
        _single: SingularRequest,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);
        //println!("{:#?}", _data);

        let _response = OrderView::report_all(&_conn)?;

        // let _response =
        //     UserView::create_item(&_conn, &form, &_single.fields, &_single.expand).await?;

        let _body = serde_json::to_string(&_response)?;
        Ok(HttpResponse::Created()
            .content_type(ContentType::json())
            .body(_body))
    }
}
