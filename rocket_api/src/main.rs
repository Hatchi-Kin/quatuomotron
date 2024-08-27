#[macro_use] extern crate rocket;

use rusqlite::{params, Connection, Result};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rocket::serde::{json::Json, Deserialize, Serialize};
use chrono::Utc;
use std::sync::Mutex;
use rocket::State;
use rocket::{get, launch, routes, Build, Rocket};
use rocket::response::content::RawHtml;
use rocket::fs::NamedFile;
use std::path::PathBuf;

// Struct representing a group of people
#[derive(Serialize, Deserialize)]
struct Group {
    members: Vec<Person>,
}

// Struct representing a person
#[derive(Serialize, Deserialize, Clone)]
struct Person {
    nom: String,
    prenom: String,
    email: String,
}

// Struct for managing database connection
struct DbConn {
    conn: Mutex<Connection>,
}

// Route handler for the root path
#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml("<div style='text-align: center;'><h1>🚀 Welcome to the Rocket Quatuomotron API!</h1></div>")
}

// Route handler to serve the favicon
#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(PathBuf::from("static/favicon.ico")).await.ok()
}

// Route handler to get the count of people in the database
#[get("/count")]
fn get_count(state: &State<DbConn>) -> Result<Json<i64>, String> {
    let conn = state.conn.lock().map_err(|_| "Failed to acquire lock".to_string())?;
    let count = conn.prepare("SELECT COUNT(*) FROM person")
        .map_err(|e| e.to_string())?
        .query_row([], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    Ok(Json(count))
}

// Route handler to get all people in the database
#[get("/people")]
fn get_people(state: &State<DbConn>) -> Result<Json<Vec<Person>>, String> {
    let conn = state.conn.lock().map_err(|_| "Failed to acquire lock".to_string())?;
    let mut stmt = conn.prepare("SELECT nom, prenom, email FROM person").map_err(|e| e.to_string())?;
    let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
    let mut people = Vec::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        let person = Person {
            nom: row.get(0).map_err(|e| e.to_string())?,
            prenom: row.get(1).map_err(|e| e.to_string())?,
            email: row.get(2).map_err(|e| e.to_string())?,
        };
        people.push(person);
    }
    Ok(Json(people))
}

// Route handler to generate groups of people
#[get("/groups/<group_size>")]
fn generate_groups(state: &State<DbConn>, group_size: usize) -> Result<Json<Vec<Group>>, String> {
    let conn = state.conn.lock().map_err(|_| "Failed to acquire lock".to_string())?;
    let mut stmt = conn.prepare("SELECT nom, prenom, email FROM person").map_err(|e| e.to_string())?;
    let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
    let mut people = Vec::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        let person = Person {
            nom: row.get(0).map_err(|e| e.to_string())?,
            prenom: row.get(1).map_err(|e| e.to_string())?,
            email: row.get(2).map_err(|e| e.to_string())?,
        };
        people.push(person);
    }

    // Shuffle the list of people to randomize the groups
    let mut rng = thread_rng();
    people.shuffle(&mut rng);

    // Split the people into groups of the specified size
    let groups: Vec<Group> = people.chunks(group_size).map(|chunk| Group { members: chunk.to_vec() }).collect();
    
    // Return the groups as a JSON response
    Ok(Json(groups))
}

// Route handler to save groups of people to the database
#[post("/save_groups", data = "<groups>")]
fn save_groups(state: &State<DbConn>, groups: Json<Vec<Group>>) -> Result<String, String> {
    let conn = state.conn.lock().map_err(|_| "Failed to acquire lock".to_string())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS groups (
            id INTEGER PRIMARY KEY,
            datetime TEXT NOT NULL,
            group_data TEXT NOT NULL
        )",
        [],
    ).map_err(|e| e.to_string())?;

    let datetime = Utc::now().to_rfc3339();
    for group in groups.into_inner() {
        let group_data = serde_json::to_string(&group).map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO groups (datetime, group_data) VALUES (?1, ?2)",
            params![datetime, group_data],
        ).map_err(|e| e.to_string())?;
    }

    Ok("Groups saved to the database.".to_string())
}

// Rocket launch function to start the server and mount the routes
#[launch]
fn rocket() -> Rocket<Build> {
    let conn = Connection::open("people.db").expect("Failed to open database");
    rocket::build()
        .manage(DbConn { conn: Mutex::new(conn) })
        .mount("/", routes![index, favicon, get_count, get_people, generate_groups, save_groups])
}