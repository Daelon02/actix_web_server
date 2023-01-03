#[macro_use]
extern crate diesel;

use crate::auth::validator;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

mod auth;
mod errors;
mod handlers;
mod models;
mod schema;

pub type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start http server
    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(auth)
            .data(pool.clone())
            .route("/tickets", web::get().to(handlers::get_tickets))
            .route("/tickets/{id}", web::get().to(handlers::get_ticket_by_id))
            .route("/tickets", web::post().to(handlers::add_ticket))
            .route("/tickets/{id}", web::delete().to(handlers::delete_ticket))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
