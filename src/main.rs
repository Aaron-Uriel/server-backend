#[macro_use]
extern crate diesel;

mod schema;
mod models;

use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

use dotenv::dotenv;
use std::env;

mod mylib;

#[get("/food_list")]
async fn food_list(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get()
        .expect("Could not connect to db with pool");
    
        let food_vec = web::block(move || mylib::get_food_vec(&conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            });

    match food_vec {
        Ok(vec) => HttpResponse::Ok().body(serde_json::to_string_pretty(&vec).unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[get("/variants_list")]
async fn variants_list(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get()
        .expect("Could not connect to db with pool");
    
    let food_vec = 
        web::block(move || mylib::get_variants_vec(&conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            });

    match food_vec {
        Ok(vec) => HttpResponse::Ok().body(serde_json::to_string_pretty(&vec).unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[get("/tables_table_list")]
async fn tables_list(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get()
        .expect("Could not connect to db with pool");
    
    let food_vec = 
        web::block(move || mylib::get_tables_vec(&conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            });

    match food_vec {
        Ok(vec) => HttpResponse::Ok().body(serde_json::to_string_pretty(&vec).unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[post("/new_client")]
async fn new_client(pool: web::Data<DbPool>, data: web::Json<models::NewClient>) -> HttpResponse {
    let conn = pool.get()
        .expect("Impossible to connect to db");

    let new_client = 
        web::block(move || mylib::insert_client(&conn, data.into_inner()))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            });
    
    match new_client {
        Ok(client) => HttpResponse::Created().body(serde_json::to_string_pretty(&client).unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();

    let ip_address = match env::var("SERVER_IP") {
        Ok(var) => var,
        Err(e) => panic!("{}", e)
    };

    let pool = mylib::create_pool();

    HttpServer::new(
        move || {
            App::new()
                .data(pool.clone())
                .service(food_list)
                .service(tables_list)
                .service(variants_list)
                .service(new_client)
        }
    ).bind(ip_address)?
    .run()
    .await
}