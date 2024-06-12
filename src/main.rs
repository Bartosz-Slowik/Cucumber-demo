mod models;
mod product_controllers;
mod product_services;

use actix_web::{web, App, HttpServer,http};
use actix_cors::Cors;

use mongodb::Client;
use std::env;

use product_controllers::{
    create_product, delete_product, get_product, get_product_history, get_products, update_product,
};

const PRODUCT_COLLECTION_NAME: &str = "Product";
const HISTORY_COLLECTION_NAME: &str = "ProductHistory";
const DB_NAME: &str = "ZTP";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    println!("Actix Rust CRUD Example... With MongoDB");

    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client = Client::with_uri_str(&mongodb_uri)
        .await
        .expect("failed to connect to db");

    HttpServer::new(move || {
        App::new().wrap(
            Cors::default()
                .allowed_origin("http://localhost:3000")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
                .max_age(3600),
        )
            .app_data(web::Data::new(client.clone()))
            .service(get_product)
            .service(get_products)
            .service(get_product_history)
            .service(create_product)
            .service(delete_product)
            .service(update_product)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
