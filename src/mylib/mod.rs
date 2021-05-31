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

pub fn insert_order(conn: &MysqlConnection, foods_to_order: Vec<models::Food>, client: models::Client) {
    use schema::orders::dsl::*;
    
    let mut order_data: Vec<models::NewOrder> = Vec::new();

    for food in foods_to_order.iter() {
        order_data.push(models::NewOrder {
            client_id: client.id,
            food_id: food.id,
            food_amount: 1,
            variant_id: 1 //Esto se debe editar después
        });
    }

    diesel::insert_into(orders)
        .values(order_data)
        .execute(conn)
        .expect(&format!("No se pudo insertar a la tabla de ordenes la comida del cliente {}", client.name));
}
