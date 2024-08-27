#[macro_use] extern crate rocket;

mod db;

use rocket::{get, post, routes, Build, Rocket, State};
use rocket::http::Header;
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::Request;
use rocket::Response;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rusqlite::Connection; // Import rusqlite::Connection
use std::sync::Mutex; // Import std::sync::Mutex
use db::{DbConn, Person, Group};

// Fairing pour ajouter les en-têtes CORS
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    }
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml("<div style='text-align: center;'><h1>🚀 Welcome to the Rocket Quatuomotron API!</h1></div>")
}

#[get("/count")]
fn get_count(state: &State<DbConn>) -> Result<Json<i64>, String> {
    state.get_count().map(Json).map_err(|e| e.to_string())
}

#[get("/people")]
fn get_people(state: &State<DbConn>) -> Result<Json<Vec<Person>>, String> {
    state.get_people().map(Json).map_err(|e| e.to_string())
}

#[get("/groups/<group_size>")]
fn generate_groups(state: &State<DbConn>, group_size: usize) -> Result<Json<Vec<Group>>, String> {
    let mut people = state.get_people()?;
    
    // Shuffle the list of people to randomize the groups
    let mut rng = thread_rng();
    people.shuffle(&mut rng);

    // Split the people into groups of the specified size
    let groups: Vec<Group> = people.chunks(group_size).map(|chunk| Group { members: chunk.to_vec() }).collect();
    
    // Return the groups as a JSON response
    Ok(Json(groups))
}

#[post("/save_groups", data = "<groups>")]
fn save_groups(state: &State<DbConn>, groups: Json<Vec<Group>>) -> Result<String, String> {
    state.save_groups(groups.into_inner()).map(|_| "Groups saved to the database.".to_string())
}

#[launch]
fn rocket() -> Rocket<Build> {
    let conn = Connection::open("people.db").expect("Failed to open database");
    rocket::build()
        .manage(DbConn { conn: Mutex::new(conn) })
        .attach(CORS) // Attacher le fairing CORS pour gérer les requêtes cross-origin
        .mount("/", routes![index, get_count, get_people, generate_groups, save_groups])
}