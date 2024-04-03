use actix_web::{delete, get, post, put, web, HttpResponse};
use crate::{models::NewProduct, product_services::{create_product_service, delete_product_service, update_product_service}};
use mongodb::Client;
use crate::product_services::{get_products_service, get_product_service, get_product_history_service};

#[get("/api/products")]
pub async fn get_products(client: web::Data<Client>) -> HttpResponse {
    match get_products_service(client).await {
        Ok(products) => {
            if !products.is_empty() {
                HttpResponse::Ok().json(products)
            } else {
                HttpResponse::NotFound().body("No products found")
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[get("/api/products/{_id}")]
pub async fn get_product(client: web::Data<Client>, _id: web::Path<String>) -> HttpResponse {
    match get_product_service(client, _id.into_inner()).await {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[get("/api/producthistory")]
pub async fn get_product_history(client: web::Data<Client>) -> HttpResponse {
    match get_product_history_service(client).await {
        Ok(history) => {
            if !history.is_empty() {
                HttpResponse::Ok().json(history)
            } else {
                HttpResponse::NotFound().body("No product history found")
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}
#[put("/api/products/{_id}")]
pub async fn update_product(client: web::Data<Client>, _id: web::Path<String>, new_product: web::Json<NewProduct>) -> HttpResponse {
    match update_product_service(client, _id.into_inner(), new_product.into_inner()).await {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[post("/api/products")]
pub async fn create_product(client: web::Data<Client>, new_product: web::Json<NewProduct>) -> HttpResponse {
    match create_product_service(client, new_product.into_inner()).await {
        Ok(product) => HttpResponse::Ok().body(product._id.to_string()),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
#[delete("/api/products/{_id}")]
pub async fn delete_product(client: web::Data<Client>, _id: web::Path<String>) -> HttpResponse {
    match delete_product_service(client, _id.into_inner()).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}