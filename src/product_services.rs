use crate::{models::{NewProduct, Product, ProductHistory, ShortProduct}, DB_NAME, HISTORY_COLLECTION_NAME, PRODUCT_COLLECTION_NAME};
use mongodb::{bson::{self, doc, oid::ObjectId}, Client, Collection};
use actix_web::web;
use futures_util::stream::StreamExt;

pub async fn get_products_service(client: web::Data<Client>) -> Result<Vec<ShortProduct>, String> {
    println!("get_products_service");
    let collection: Collection<ShortProduct> =
        client.database(DB_NAME).collection(PRODUCT_COLLECTION_NAME);
    let projection = doc! {"_id": 1, "name": 1, "price": 1};
    let mut cursor = collection
        .find(
            None,
            Some(
                mongodb::options::FindOptions::builder()
                    .projection(projection)
                    .build(),
            ),
        )
        .await
        .map_err(|_| "Can't fetch products".to_string())?;

    let mut products: Vec<ShortProduct> = Vec::new();

    while let Some(doc) = cursor.next().await {
        match doc {
            Ok(product) => {
                products.push(product);
            }
            Err(_err) => {
                return Err(_err.to_string());
            }
        }
    }

    Ok(products)
}

pub async fn get_product_service(client: web::Data<Client>, _id: String) -> Result<Product, String> {
    let object_id = match ObjectId::parse_str(&_id) {
        Ok(object_id) => object_id,
        Err(_) => return Err("Invalid ID format specified".to_string()),
    };

    let collection: Collection<Product> =
        client.database(DB_NAME).collection(PRODUCT_COLLECTION_NAME);

    match collection.find_one(doc! {"_id": object_id}, None).await {
        Ok(product) => Ok(product.unwrap()),
        Err(_err) => Err(_err.to_string()),
    }
}

pub async fn get_product_history_service(client: web::Data<Client>) -> Result<Vec<ProductHistory>, String> {
    let collection: Collection<ProductHistory> =
        client.database(DB_NAME).collection(HISTORY_COLLECTION_NAME);
    let mut cursor = collection
        .find(None, None)
        .await
        .map_err(|_| "Can't fetch product history".to_string())?;

    let mut history: Vec<ProductHistory> = Vec::new();

    while let Some(doc) = cursor.next().await {
        match doc {
            Ok(record) => {
                history.push(record);
            }
            Err(_err) => {
                return Err(_err.to_string());
            }
        }
    }

    Ok(history)
}

pub async fn update_product_service(client: web::Data<Client>, _id: String, new_product: NewProduct) -> Result<Product, String> {
    let object_id = match ObjectId::parse_str(&_id) {
        Ok(object_id) => object_id,
        Err(_) => return Err("Invalid ID format specified".to_string()),
    };

    let collection: Collection<Product> = client.database(DB_NAME).collection(PRODUCT_COLLECTION_NAME);
    let history_collection: Collection<ProductHistory> = client.database(DB_NAME).collection(HISTORY_COLLECTION_NAME);

    let old_product = collection.find_one(doc! {"_id": object_id}, None).await.unwrap();

    let mut update_doc = doc! {};
    if let Some(name) = new_product.name {
        update_doc.insert("name", name);
    }
    if let Some(description) = new_product.description {
        update_doc.insert("description", description);
    }
    if let Some(price) = new_product.price {
        update_doc.insert("price", price);
    }
    if let Some(quantity) = new_product.quantity {
        update_doc.insert("quantity", quantity);
        if quantity == 0 {
            update_doc.insert("status", "Unavailable");
        }
    }
    if let Some(status) = new_product.status {
        update_doc.insert("status", status);
    }

    let result = collection.find_one_and_update(doc! {"_id": object_id}, doc! {"$set": update_doc}, None).await;

    match result {
        Ok(product) => {
            let history = ProductHistory {
                product_id: object_id,
                changed_at:  bson::DateTime::now(),
                change_type: "UPDATE".to_string(),
                old_product,
                new_product: product.clone(),
            };
            let _ = history_collection.insert_one(history, None).await;
            Ok(product.unwrap())
        },
        Err(_err) => Err(_err.to_string())
    }
}
pub async fn create_product_service(client: web::Data<Client>, new_product: NewProduct) -> Result<Product, String> {
    let collection: Collection<Product> = client.database(DB_NAME).collection(PRODUCT_COLLECTION_NAME);
    let history_collection: Collection<ProductHistory> = client.database(DB_NAME).collection(HISTORY_COLLECTION_NAME);

    let name = match new_product.name {
        Some(name) => name,
        None => return Err("Missing field: name".to_string()),
    };

    let price = match new_product.price {
        Some(price) => price,
        None => return Err("Missing field: price".to_string()),
    };

    let quantity = match new_product.quantity {
        Some(quantity) => quantity,
        None => return Err("Missing field: quantity".to_string()),
    };

    let product = Product {
        _id: ObjectId::new(),
        name,
        description: new_product.description.unwrap_or_default(),
        price,
        quantity,
        status: if quantity == 0 {
            "Unavailable".to_string()
        } else {
            new_product.status.unwrap_or("Available".to_string())
        },
    };

    let result = collection.insert_one(&product, None).await;

    match result {
        Ok(_) => {
            let history = ProductHistory {
                product_id: product._id,
                changed_at: bson::DateTime::now(),
                change_type: "CREATE".to_string(),
                old_product: None,
                new_product: Some(product.clone()),
            };
            let _ = history_collection.insert_one(history, None).await;
            Ok(product)
        },
        Err(_err) => Err(_err.to_string()),
    }
}
pub async fn delete_product_service(client: web::Data<Client>, _id: String) -> Result<String, String> {
    let object_id = match ObjectId::parse_str(&_id) {
        Ok(object_id) => object_id,
        Err(_) => return Err("Invalid ID format specified".to_string()),
    };

    let collection: Collection<Product> = client.database(DB_NAME).collection(PRODUCT_COLLECTION_NAME);
    let history_collection: Collection<ProductHistory> = client.database(DB_NAME).collection(HISTORY_COLLECTION_NAME);

    let old_product = collection.find_one(doc! {"_id": object_id}, None).await.unwrap();

    match collection.delete_one(doc! {"_id": object_id}, None).await {
        Ok(_) => {
            let history = ProductHistory {
                product_id: object_id,
                changed_at: bson::DateTime::now(),
                change_type: "DELETE".to_string(),
                old_product,
                new_product: None,
            };
            let _ = history_collection.insert_one(history, None).await;
            Ok("Product deleted".to_string())
        },
        Err(_err) => Err(_err.to_string()),
    }
}