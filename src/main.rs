mod models;
mod product_controllers;
mod product_services;

use actix_web::{web, App, HttpServer};
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
        App::new()
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
