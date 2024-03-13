#![feature(decl_macro)]

use rocket::get;

#[get("/")]
fn index() -> &'static str {
    "Byebye, world!"
}

fn main() {
    rocket::build().mount("/", routes![index]).launch();
}
