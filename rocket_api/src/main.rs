#[macro_use] extern crate rocket;

use rocket::{get, post, routes, Build, Rocket, State};
use rocket::http::Header;
use rocket::response::content::RawHtml;
use rocket::serde::{json::Json, Serialize, Deserialize};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::Request;
use rocket::Response;
use rusqlite::{params, Connection};
use std::sync::Mutex;
use rand::seq::SliceRandom;
use rand::thread_rng;
use chrono::Utc;

#[derive(Serialize, Deserialize, Clone)]
struct Person {
    nom: String,
    prenom: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct Group {
    members: Vec<Person>,
}

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

// Struct pour gérer la connexion à la base de données
struct DbConn {
    conn: Mutex<Connection>,
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml("<div style='text-align: center;'><h1>🚀 Welcome to the Rocket Quatuomotron API!</h1></div>")
}

#[get("/count")]
fn get_count(state: &State<DbConn>) -> Result<Json<i64>, String> {
    let conn = state.conn.lock().map_err(|_| "Failed to acquire lock".to_string())?;
    let count = conn.prepare("SELECT COUNT(*) FROM person")
        .map_err(|e| e.to_string())?
        .query_row([], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    Ok(Json(count))
}

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

#[launch]
fn rocket() -> Rocket<Build> {
    let conn = Connection::open("people.db").expect("Failed to open database");
    rocket::build()
        .manage(DbConn { conn: Mutex::new(conn) })
        .attach(CORS) // Attacher le fairing CORS pour gérer les requêtes cross-origin
        .mount("/", routes![index, get_count, get_people, generate_groups, save_groups])
}