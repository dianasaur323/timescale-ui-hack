#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate dotenv;
extern crate postgres;

use postgres::{Connection, TlsMode};
use std::env;
// #[macro_use] extern crate rocket_contrib;

// use rocket_contrib::databases::diesel;
// use diesel::sql_query;
// use crate::rocket_contrib::databases::diesel::RunQueryDsl;

// #[database("postgres_db")]
// struct PostgresDB(diesel::PgConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// #[get("/")]
// fn index(db: PostgresDB) -> &'static str {
//     let _ = sql_query("SELECT 1").load(&db);
// }

// #[get("/<name>/<age>")]
// fn hello(name: String, age: u8) -> String {
//     format!("Hello, {} year old named {}!", age, name)
// }

fn main() {
    // Pull in defined database url from .env file
    dotenv::dotenv().expect("Failed to read .env file");
    let key = "DATABASE_URL";
    let db_url = match env::var(key) {
            Ok(val) => val,
            Err(_e) => panic!("DATABASE_URL is not defined in the environment"),
    };

    let conn = Connection::connect(db_url, TlsMode::None)
            .unwrap();

    let table_query = "select * from information_schema.tables where table_schema
        not in ('pg_catalog','_timescaledb_catalog','_timescaledb_config','_timescaledb_internal',
        '_timescaledb_cache','information_schema','timescaledb_information');";
    let mut tables:Vec<(String, String)> = Vec::new();
    for row in &conn.query(table_query, &[]).unwrap() {
        tables.push((row.get("table_schema"),row.get("table_name")));
    }

    let hypertable_query = "select * from timescaledb_information.hypertable;";
    let mut hypertables:Vec<(String, String, i32, String)> = Vec::new();
    for row in &conn.query(hypertable_query, &[]).unwrap() {
        hypertables.push((row.get("table_schema"),row.get("table_name"),row.get("num_chunks"),row.get("total_size")))
    }


    rocket::ignite()
        // .attach(PostgresDB::fairing())
        .mount("/", routes![index])
        // .mount("/hello", routes![hello])
        .launch();
}
