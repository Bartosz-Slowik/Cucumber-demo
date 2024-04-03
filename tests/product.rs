use cucumber::{given, then, when, World as _};
use reqwest::Client;
use serde_json::json;

#[derive(cucumber::World, Debug, Default)]
pub struct ProductWorld {
    client: Client,
    product: Option<serde_json::Value>,
    response_status: Option<u16>,
    response_body: Option<String>,
}

#[given(expr = "I have a product with name {string}, price {float}, description {string}, quantity {int}, and status {string}")]
pub async fn i_have_a_product(
    world: &mut ProductWorld,
    name: String,
    price: i64,
    description: String,
    quantity: i32,
    status: String,
) {
    let product = json!({
        "name": name,
        "price": price,
        "description": description,
        "quantity": quantity,
        "status": status
    });
    print!("{:?}", product);
    world.product = Some(product);
}

#[when("I create the product")]
pub async fn i_create_the_product(world: &mut ProductWorld) {
    let product = world.product.as_ref().unwrap();

    let response = world
        .client
        .post("http://localhost:8080/api/products")
        .json(product)
        .send()
        .await
        .unwrap();

    let status = response.status().as_u16();
    let body = response.text().await.unwrap();

    world.response_status = Some(status);
    world.response_body = Some(body);
}

#[then("the product should be created")]
pub async fn the_product_should_be_created(world: &mut ProductWorld) {
    let status = world.response_status.unwrap();
    assert_eq!(status, 200);
}

#[then("the product should not be created and I should receive an error")]
pub async fn the_product_should_not_be_created_and_i_should_receive_an_error(
    world: &mut ProductWorld,
) {
    let status = world.response_status.unwrap();
    assert_ne!(status, 200);
}

#[when("I delete the product")]
pub async fn i_delete_the_product(world: &mut ProductWorld) {
    let product_id = world.response_body.as_ref().unwrap();
    dbg!(product_id);

    let response = world
        .client
        .delete(format!("http://localhost:8080/api/products/{}", product_id))
        .send()
        .await
        .unwrap();

    let status = response.status().as_u16();
    let body = response.text().await.unwrap();

    world.response_status = Some(status);
    world.response_body = Some(body);
}

#[then("the product should be deleted")]
pub async fn the_product_should_be_deleted(world: &mut ProductWorld) {
    let status = world.response_status.unwrap();
    assert_eq!(status, 200);
}
#[when("I get the list of products")]
pub async fn i_get_the_list_of_products(world: &mut ProductWorld) {
    let response = world
        .client
        .get("http://localhost:8080/api/products")
        .send()
        .await
        .unwrap();

    let status = response.status().as_u16();
    let body = response.text().await.unwrap();

    world.response_status = Some(status);
    world.response_body = Some(body);
}

#[then("the list of products should be retrieved")]
pub async fn the_list_of_products_should_be_retrieved(world: &mut ProductWorld) {
    let status = world.response_status.unwrap();
    assert_eq!(status, 200);
}
#[when("I get the product")]
pub async fn i_get_the_product(world: &mut ProductWorld) {
    let product_id = world.response_body.as_ref().unwrap();

    let response = world
        .client
        .get(format!("http://localhost:8080/api/products/{}", product_id))
        .send()
        .await
        .unwrap();

    let status = response.status().as_u16();
    let body = response.text().await.unwrap();

    world.response_status = Some(status);
    world.response_body = Some(body);
}

#[then("the product should be retrieved")]
pub async fn the_product_should_be_retrieved(world: &mut ProductWorld) {
    let status = world.response_status.unwrap();
    assert_eq!(status, 200);
}

#[when("I get the product's history")]
pub async fn i_get_the_products_history(world: &mut ProductWorld) {
    let response = world
        .client
        .get("http://localhost:8080/api/producthistory".to_string())
        .send()
        .await
        .unwrap();

    let status = response.status().as_u16();
    let body = response.text().await.unwrap();

    world.response_status = Some(status);
    world.response_body = Some(body);
}

#[then("the product's history should be retrieved")]
pub async fn the_products_history_should_be_retrieved(world: &mut ProductWorld) {
    let status = world.response_status.unwrap();
    assert_eq!(status, 200);
}

#[tokio::main]
async fn main() {
    ProductWorld::run("tests/features/prod.features").await;
}
