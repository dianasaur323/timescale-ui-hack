#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate dotenv;
extern crate postgres;

extern crate rocket_cors;

use postgres::{Connection, TlsMode};
use std::env;
// use rocket::response::content;
use serde::{Serialize, Deserialize};

use rocket::http::Method; // 1.

use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error, // 2.
    Cors, CorsOptions // 3.
};

// #[database("postgres_db")]
// struct PostgresDB(diesel::PgConnection);

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

#[derive(Serialize, Deserialize, Debug)]
struct Table {
    table_schema: String,
    table_name: String,
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

// select t1.table_schema, t1.table_name,
// select * from information_schema.tables as t1
// LEFT OUTER JOIN timescaledb_information.hypertable as t2 on t1.table_schema = t2.table_schema and t1.table_name = t2.table_schema
// where t1.table_schema not in ('pg_catalog','_timescaledb_catalog','_timescaledb_config','_timescaledb_internal','_timescaledb_cache','information_schema','timescaledb_information');

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

fn make_cors() -> Cors {
    // let allowed_origins = AllowedOrigins::some_exact(&[ // 4.
    //     "http://localhost:8080",
    //     "http://127.0.0.1:8080",
    //     "http://localhost:8000",
    //     "http://0.0.0.0:8000",
    //     AccessControlAllowOrigin::Null,
    // ]);

    let allowed_origins = AllowedOrigins::all();

    CorsOptions { // 5.
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(), // 1.
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin", // 6.
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

fn main() {
    // Pull in defined database url from .env file

    rocket::ignite()
        .attach(make_cors())
        .mount("/", routes![index])
        .mount("/hypertable", routes![hypertable])
        // .mount("/hello", routes![hello])
        .launch();
}
