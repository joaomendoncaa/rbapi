#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    println!("Application ignited!");

    rocket::ignite().mount("/", routes![index]).launch();
}
