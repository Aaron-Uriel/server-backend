extern crate diesel;

use chrono::prelude::*;
use crate::schema::{
    clients,
    orders
};
use bigdecimal::BigDecimal;

use serde::{
    Serialize,
    Deserialize
};

#[derive(Queryable, Debug, Serialize)]
pub struct Client {
    pub id: u32,
    pub table_id: Option<u8>,
    pub arrival: NaiveDateTime
}

#[derive(Queryable, Debug, Serialize)]
pub struct Food {
    pub id: u16,
    pub name: String,
    pub price: BigDecimal,
    pub est_time: Option<u8>,
    pub has_variants: bool
}

#[derive(Insertable, Default, Deserialize)]
#[table_name="clients"]
pub struct NewClient {
    pub table_id: Option<u8>
}

#[derive(Insertable, Debug, Deserialize)]
#[table_name="orders"]
pub struct NewOrder {
    pub client_id: u32,
    pub food_id: u16,
    pub food_amount: u8,
    pub variant_id: u8
}

#[derive(Queryable, Debug)]
pub struct Order {
    pub order_id: u32,
    pub client_id: u32,
    pub food_id: u16,
    pub food_amount: u8,
    pub variant_id: u8,
    pub is_finished: bool
}

#[derive(Queryable, Serialize)]
pub struct Variant {
    pub id: u8,
    pub full_name: String
}

#[derive(Queryable, Serialize)]
pub struct Table {
    id: u8,
    is_occupied: bool
}