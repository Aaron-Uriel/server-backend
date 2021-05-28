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

#[derive(Queryable, Debug)]
pub struct Client {
    pub id: u32,
    pub name: String,
    pub arrival: NaiveDateTime
}

#[derive(Queryable, Debug, Serialize)]
pub struct Food {
    pub id: u16,
    pub name: String,
    pub price: BigDecimal,
    pub est_time: Option<u8>,
    pub var_group: u8
}

#[derive(Insertable, Default)]
#[table_name="clients"]
pub struct NewClient {
    pub table_id: u8
}

#[derive(Insertable, Debug, Default)]
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
    pub food_amount: u32,
    pub variant_id: String,
    pub is_finished: bool
}

#[derive(Queryable)]
pub struct Variant {
    pub id: u8,
    pub full_name: String,
    pub group: u8
}