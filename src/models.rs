use serde::{Deserialize, Serialize};
use mongodb::bson::{doc, oid::ObjectId, DateTime};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Product {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub price: u32,
    pub quantity: u32,
    pub status: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct NewProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<u32>,
    pub quantity: Option<u32>,
    pub status: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ShortProduct {
    pub _id: ObjectId,
    pub name: String,
    pub price: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductHistory {
    pub product_id: ObjectId,
    pub changed_at: DateTime,
    pub change_type: String,
    pub old_product: Option<Product>,
    pub new_product: Option<Product>,
}
