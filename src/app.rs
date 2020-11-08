use std::sync::Arc;

use crate::config;
use crate::graphql::{Context, Schema};

use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;
use actix_web::{web, Error, HttpResponse};

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

pub fn graphiql() -> HttpResponse {
    let graphql_endpoint_url = config::GRAPHIQL_TARGET_ENDPOINT() + "/graphql";
    HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(graphiql_source(&graphql_endpoint_url))
}

pub async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
    pool: web::Data::<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let ctx = &Context {connection: pool.clone()};
        let res = data.execute(&st, ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(resp))
}
