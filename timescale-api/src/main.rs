#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate dotenv;
extern crate postgres;

use postgres::{Connection, TlsMode};
use std::env;
// use rocket::response::content;
use serde::{Serialize, Deserialize};

// #[database("postgres_db")]
// struct PostgresDB(diesel::PgConnection);

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

#[derive(Serialize, Deserialize, Debug)]
struct Table {
    table_schema: String,
    table_name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Hypertable {
    chunk_id: i32,
    ranges: String,
    total_size: i32,
}

#[get("/<query_type>")]
fn index(query_type: String) -> String{
    // content::Json(format!("Hello, {} year old named!", query_type).tostring())
    dotenv::dotenv().expect("Failed to read .env file");
    let key = "DATABASE_URL";
    let db_url = match env::var(key) {
            Ok(val) => val,
            Err(_e) => panic!("DATABASE_URL is not defined in the environment"),
    };

    let conn = Connection::connect(db_url, TlsMode::None)
            .unwrap();

    let query = match query_type.as_ref() {
        "all_tables" => "select * from information_schema.tables where table_schema
            not in ('pg_catalog','_timescaledb_catalog','_timescaledb_config','_timescaledb_internal',
            '_timescaledb_cache','information_schema','timescaledb_information');",
        "all_hypertables" => "select * from timecaledb_information.hypertable;",
        // "hypertable" => {
        //     let formatted_query = format!("select * from chunk_relation_size_pretty('{}');", hypertable_name);
        //     return formatted_query;
        // },
        _ => "Error: incorrect query type",
    };

    let mut tables:Vec<Table> = Vec::new();
    match query_type.as_ref() {
        "all_tables" =>
            for row in &conn.query(query, &[]).unwrap() {
                let table = Table { table_schema: row.get("table_schema"), table_name: row.get("table_name") };
                tables.push(table);
            }
        _ => (),
    }

    return serde_json::to_string(&tables).unwrap()
}

// create a hypertable? show the schema? SELECT create_hypertable('conditions', 'time');

#[get("/hypertable/<hypertable_name>")]
fn hypertable(hypertable_name: String) -> String{
    // content::Json(format!("Hello, {} year old named!", hypertable_name).tostring())
    dotenv::dotenv().expect("Failed to read .env file");
    let key = "DATABASE_URL";
    let db_url = match env::var(key) {
            Ok(val) => val,
            Err(_e) => panic!("DATABASE_URL is not defined in the environment"),
    };

    let conn = Connection::connect(db_url, TlsMode::None)
            .unwrap();

    // let query = format!("select * from chunk_relation_size_pretty('{}');", hypertable_name);

    let mut chunks:Vec<Hypertable> = Vec::new();
    for row in &conn.query("select * from chunk_relation_size_pretty('$1')", &[&hypertable_name]).unwrap() {
        let chunk = Hypertable { chunk_id: row.get("chunk_id"),
                            ranges: row.get("ranges"),
                            total_size: row.get("total_size"),
                    };
        chunks.push(chunk);
    }

    return serde_json::to_string(&chunks).unwrap()
}

fn main() {
    // Pull in defined database url from .env file

    // let hypertable_query = "select * from timescaledb_information.hypertable;";
    // let mut hypertables:Vec<(String, String, i32, String)> = Vec::new();
    // for row in &conn.query(hypertable_query, &[]).unwrap() {
    //     hypertables.push((row.get("table_schema"),row.get("table_name"),row.get("num_chunks"),row.get("total_size")))
    // }


    rocket::ignite()
        // .attach(PostgresDB::fairing())
        .mount("/", routes![index])
        .mount("/hypertable", routes![hypertable])
        // .mount("/hello", routes![hello])
        .launch();
}
