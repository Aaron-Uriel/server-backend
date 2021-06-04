use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::*;

pub fn create_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool")
}

pub fn get_food_vec(conn: &MysqlConnection) -> Result<Vec<models::Food>, diesel::result::Error> {
    use schema::foods::dsl::*;

    let result = foods.load::<models::Food>(conn)
        .expect("Error while getting food list from db");
    
    Ok(result)
}

pub fn get_variants_vec(conn: &MysqlConnection) -> Result<Vec<models::Variant>, diesel::result::Error> {
    use schema::variants::dsl::*;

    let result = variants.load::<models::Variant>(conn)
        .expect("Impossible to get variants from db.");
    
    Ok(result)
}

pub fn get_tables_vec(conn: &MysqlConnection) -> Result<Vec<models::Table>, diesel::result::Error> {
    use schema::tables::dsl::*;

    let result = tables.load::<models::Table>(conn)
        .expect("Impossible to get tables table from db.");
    
    Ok(result)
}

pub fn insert_client(conn: &MysqlConnection, client: models::NewClient) -> Result<models::Client, diesel::result::Error> {
    use schema::clients::dsl;

    diesel::insert_into(dsl::clients)
        .values(client)
        .execute(conn)
        .unwrap();
    
    dsl::clients.order(dsl::arrival.desc())
        .first::<models::Client>(conn)
}

pub fn insert_order(conn: &MysqlConnection, orders_vector: Vec<models::NewOrder>) -> Result<usize, diesel::result::Error> {
    use schema::orders::dsl::*;

    diesel::insert_into(orders)
        .values(orders_vector)
        .execute(conn)
}
