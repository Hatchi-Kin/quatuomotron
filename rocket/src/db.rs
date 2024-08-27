use rusqlite::{params, Connection};
use std::sync::Mutex;
use rocket::serde::{Serialize, Deserialize};
use chrono::Utc;

#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
    pub nom: String,
    pub prenom: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub members: Vec<Person>,
}

// Struct for managing the database connection
pub struct DbConn {
    pub conn: Mutex<Connection>,
}

impl DbConn {
    pub fn get_count(&self) -> Result<i64, String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire lock".to_string())?;
        let count = conn.prepare("SELECT COUNT(*) FROM person")
            .map_err(|e| e.to_string())?
            .query_row([], |row| row.get(0))
            .map_err(|e| e.to_string())?;
        Ok(count)
    }

    pub fn get_people(&self) -> Result<Vec<Person>, String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire lock".to_string())?;
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
        Ok(people)
    }

    pub fn save_groups(&self, groups: Vec<Group>) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire lock".to_string())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS groups (
                id INTEGER PRIMARY KEY,
                datetime TEXT NOT NULL,
                group_data TEXT NOT NULL
            )",
            [],
        ).map_err(|e| e.to_string())?;

        let datetime = Utc::now().to_rfc3339();
        for group in groups {
            let group_data = serde_json::to_string(&group).map_err(|e| e.to_string())?;
            conn.execute(
                "INSERT INTO groups (datetime, group_data) VALUES (?1, ?2)",
                params![datetime, group_data],
            ).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    pub fn get_unique_datetimes(&self) -> Result<Vec<String>, String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire lock".to_string())?;
        let mut stmt = conn.prepare("SELECT DISTINCT datetime FROM groups").map_err(|e| e.to_string())?;
        let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
        let mut datetimes = Vec::new();
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let datetime: String = row.get(0).map_err(|e| e.to_string())?;
            datetimes.push(datetime);
        }
        Ok(datetimes)
    }

    pub fn get_groups_by_datetime(&self, datetime: &str) -> Result<Vec<Group>, String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire lock".to_string())?;
        let mut stmt = conn.prepare("SELECT group_data FROM groups WHERE datetime = ?1").map_err(|e| e.to_string())?;
        let mut rows = stmt.query(params![datetime]).map_err(|e| e.to_string())?;
        let mut groups = Vec::new();
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let group_data: String = row.get(0).map_err(|e| e.to_string())?;
            let group: Group = serde_json::from_str(&group_data).map_err(|e| e.to_string())?;
            groups.push(group);
        }
        Ok(groups)
    }
}