#![feature(proc_macro_hygiene, decl_macro)]

use rusqlite::Connection;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/user/<name>")]
fn endpoint_handler_user(name: String) -> String {
    let response: String = format!("Hello, {}!", &name);
    response
}

fn db_connect() {
    let db_connection: Connection =
        rusqlite::Connection::open("data.sqlite").expect("Error connecting to the database!");

    db_connection
        .execute(
            "create table if not exists main (id integer primary key);",
            [],
        )
        .expect("Couldn't create the table");
}

fn main() {
    db_connect();

    rocket::ignite()
        .mount("/", routes![index, endpoint_handler_user])
        .launch();
}
