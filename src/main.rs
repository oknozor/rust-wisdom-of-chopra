#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod tests;
mod chopra_gen;

use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize)]
struct Message {
    contents: String,
}

#[get("/", format = "json")]
fn get() -> Option<Json<Message>> {
    let message = Message {
        contents: chopra_gen::get_random_quote(),
    };
    Some(Json(message))
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![get])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
