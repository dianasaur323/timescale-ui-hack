#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
// #[macro_use] extern crate postgres;

// use postgres::{Connection, TlsMode};
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
    // let db_url = env::var(DATABASE_URL)
    // let db_url = match env::var("DATABASE_URL") {
    //     Some(val) => val.into_string().unwrap(),
    //     None => panic!("DATABASE_URL is not defined in the environment")
    // };
    let key = "DATABASE_URL";
    match env::var(key) {
            Ok(val) => println!("{}: {}", key, val),
            Err(e) => println!("Couldn't print env var {}: {}", key, e),
    }

    // print!(format!("{}",db_url));
    // let conn = Connection::connect(&db_url, TlsMode::None)
            // .unwrap();
    // conn.execute

    rocket::ignite()
        // .attach(PostgresDB::fairing())
        .mount("/", routes![index])
        // .mount("/hello", routes![hello])
        .launch();
}
