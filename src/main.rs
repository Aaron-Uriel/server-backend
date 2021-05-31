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

mod consts;
use consts::*;

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
        Ok(vec) => {
            HttpResponse::Ok().body(serde_json::to_string_pretty(&vec).unwrap())
        },
        Err(e) => {
            HttpResponse::InternalServerError();
            panic!("The the food vector should be returned, but something happened");
        }
    }
}

#[get("/variants_list")]
async fn variants_list(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get()
        .expect("Could not connect to db with pool");
    
    let food_vec = web::block(move || mylib::get_variants_vec(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        });

    match food_vec {
        Ok(vec) => {
            HttpResponse::Ok().body(serde_json::to_string_pretty(&vec).unwrap())
        },
        Err(e) => {
            HttpResponse::InternalServerError();
            panic!("The the food vector should be returned, but something happened");
        }
    }
}

#[get("/tables_table_list")]
async fn tables_list(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get()
        .expect("Could not connect to db with pool");
    
    let food_vec = web::block(move || mylib::get_tables_vec(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        });

    match food_vec {
        Ok(vec) => {
            HttpResponse::Ok().body(serde_json::to_string_pretty(&vec).unwrap())
        },
        Err(e) => {
            HttpResponse::InternalServerError();
            panic!("The the food vector should be returned, but something happened");
        }
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
        }
    ).bind(ip_address)?
    .run()
    .await
}