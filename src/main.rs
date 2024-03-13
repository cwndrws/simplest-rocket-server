#![feature(decl_macro)]

use rocket::*;

#[get("/")]
fn index() -> &'static str {
    "Byebye, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
