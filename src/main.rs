#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::json::Json;
use rusqlite::Connection;
use serde::Serialize;
use std::fs;

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
struct TodoItem {
    id: i64,
    item: String,
}
#[derive(Serialize)]
struct TodoList {
    items: Vec<TodoItem>,
}

#[derive(Serialize)]
struct StatusMessage {
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/todos")]
fn endpoint_get_todos() -> Result<Json<TodoList>, String> {
    let db_connection: Connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let mut statement = match db_connection.prepare("select id, item from todo_list;") {
        Ok(statement) => statement,
        Err(_) => return Err(String::from("Failed preparing the query statement")),
    };

    let results = statement.query_map([], |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            item: row.get(1)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();

            match collection {
                Ok(items) => Ok(Json(TodoList { items })),
                Err(_) => return Err(String::from("Couldn't collect todo items..")),
            }
        }
        Err(_) => return Err(String::from("Error getting the todo items..")),
    }
}

#[post("/todos", format = "json", data = "<item>")]
fn endpoint_post_todos(item: Json<String>) -> Result<Json<StatusMessage>, String> {
    let db_connection: Connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let mut statement =
        match db_connection.prepare("insert into todo_list (id, item) values (null, $1);") {
            Ok(statement) => statement,
            Err(e) => return Err(e.to_string()),
        };

    let results = statement.execute(&[&item.0]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows inserted!", rows_affected),
        })),
        Err(_) => return Err(String::from("Failed to insert todo item..")),
    }
}

#[delete("/todos/<id>")]
fn endpoint_delete_todos(id: i64) -> Result<Json<StatusMessage>, String> {
    let db_connection: Connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let mut statement = match db_connection.prepare("delete from todo_list where id = $1;") {
        Ok(statement) => statement,
        Err(_) => return Err(String::from("Failed preparing the query statement")),
    };

    let results = statement.execute(&[&id]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows deleted!", rows_affected),
        })),
        Err(_) => return Err(String::from("Failed to delete todo item..")),
    }
}

fn db_init() {
    let db_connection: Connection =
        Connection::open("data.sqlite").expect("Error connecting to the database!");

    db_connection
        .execute(
            "create table if not exists todo_list (
                id integer primary key,
                item varchar(64) not null
            );",
            [],
        )
        .expect("Couldn't create the table");
}

fn main() {
    let is_file_created = std::path::Path::new("data.sqlite").exists();

    if is_file_created {
        fs::remove_file("data.sqlite").expect("Couldn't delete data.sqlite file");
    }

    db_init();

    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                endpoint_get_todos,
                endpoint_post_todos,
                endpoint_delete_todos
            ],
        )
        .launch();
}
