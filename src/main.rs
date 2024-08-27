#[macro_use] extern crate rocket;

use rusqlite::{params, Connection, Result};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rocket::serde::{json::Json, Deserialize, Serialize};
use chrono::Utc;
use std::sync::Mutex;
use rocket::State;

#[derive(Serialize, Deserialize)]
struct Group {
    members: Vec<Person>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Person {
    nom: String,
    prenom: String,
    email: String,
}

struct DbConn {
    conn: Mutex<Connection>,
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

    let mut rng = thread_rng();
    people.shuffle(&mut rng);

    let groups: Vec<Group> = people.chunks(group_size).map(|chunk| Group { members: chunk.to_vec() }).collect();
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
fn rocket() -> _ {
    let conn = Connection::open("people.db").expect("Failed to open database");
    rocket::build()
        .manage(DbConn { conn: Mutex::new(conn) })
        .mount("/", routes![get_count, generate_groups, save_groups])
}