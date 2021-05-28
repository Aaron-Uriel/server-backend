use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::*;

pub fn create_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to database: {}", database_url))
}

pub fn get_food_vec(conn: &MysqlConnection) -> Vec<models::Food> {
    use schema::foods::dsl::*;

    foods.load::<models::Food>(conn)
        .expect("Error while getting food list from db")
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

pub fn get_variants_from_group(conn: &MysqlConnection, group_id: u8) -> Vec<models::Variant> {
    use schema::variants::dsl::*;

    variants.filter(v_group.eq(group_id))
        .load::<models::Variant>(conn)
        .expect("Impossible to get variants from db.")
}