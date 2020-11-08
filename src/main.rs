#[macro_use]
extern crate juniper;
#[macro_use]
extern crate diesel;

extern crate chrono;

mod app;
mod domain;
mod graphql;

use crate::graphql::{schema::create_schema, Context};
use crate::domain::connect;

use dotenv::dotenv;

use actix_web::{middleware, web, App, HttpServer};

itconfig::config! {
    DATABASE_URL: String,
    ROCKET {
        static BASE_URL: String => "/",
    },
    GRAPHIQL_TARGET_ENDPOINT: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    dotenv().ok();

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());
    let pool = connect();

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(app::graphql)))
            .service(web::resource("/graphiql").route(web::get().to(app::graphiql)))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}
