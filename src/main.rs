#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::sync::Mutex;
use std::collections::HashMap;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

// The type to represent the ID of a message.
type ID = usize;

// We're going to store all of the messages here. No need for a DB for now.
type RealEstateMap = Mutex<HashMap<ID, String>>;

#[derive(Serialize, Deserialize)]
struct RealEstate {
    id: Option<ID>,
    description: String
}

#[get("/", format = "json")]
fn getAll(map: State<RealEstateMap>) -> Json<Vec<RealEstate>> {
    let hashmap = map.lock().unwrap();
    Json(
    hashmap.iter().map(|(key, value)| {
        RealEstate {
            id: Some(*key),
            description: value.clone()
        }
    }).collect::<Vec<_>>())
}

#[get("/<id>", format = "json")]
fn get(id: ID, map: State<RealEstateMap>) -> Option<Json<RealEstate>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|contents| {
        Json(RealEstate {
            id: Some(id),
            description: contents.clone()
        })
    })
}

#[post("/<id>", format = "json", data = "<entity>")]
fn new(id: ID, entity: Json<RealEstate>, map: State<RealEstateMap>) -> JsonValue {
    let mut hashmap = map.lock().expect("map lock.");
    if hashmap.contains_key(&id) {
        json!({
            "status": "error",
            "reason": "ID exists. Try put."
        })
    } else {
        hashmap.insert(id, entity.0.description);
        json!({ "status": "ok" })
    }
}

#[put("/<id>", format = "json", data = "<entity>")]
fn update(id: ID, entity: Json<RealEstate>, map: State<RealEstateMap>) -> Option<JsonValue> {
    let mut hashmap = map.lock().unwrap();
    if hashmap.contains_key(&id) {
        hashmap.insert(id, entity.0.description);
        Some(json!({ "status": "ok" }))
    } else {
        None
    }
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
        .mount("/realestates/", routes![getAll, get, new, update])
        .register(catchers![not_found])
        .manage(Mutex::new(HashMap::<ID, String>::new()))
}

fn main() {
    rocket().launch();
}
