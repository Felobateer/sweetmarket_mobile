use std::path::Path;
use rusqlite::{Connection, params};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: Option<u32>,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub date_of_birth: Option<NaiveDate>,
    pub email: String,
    pub password: String,
}

pub struct Users {
    conn: Arc<Mutex<Connection>>,
}

impl Users {
    pub fn init() -> Result<Self, String> {
        let db_path = Path::new("backend/src/database/users_data.db");
        let conn = Connection::open(db_path).map_err(|e| format!("Failed to open DB: {}", e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS Users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
                phone_number TEXT NOT NULL,
                date_of_birth TEXT NULL,
                email TEXT NOT NULL,
                password TEXT NOT NULL
            )",
            [],
        ).map_err(|e| e.to_string())?;

        Ok(Users {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn add(&self, entry: User) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire database lock")?;
        conn.execute(
            "INSERT INTO Users (first_name, last_name, phone_number, date_of_birth, email, password) 
             VALUES (?, ?, ?, ?, ?, ?)",
            params![
                entry.first_name,
                entry.last_name,
                entry.phone_number,
                entry.date_of_birth.map(|d| d.format("%Y-%m-%d").to_string()),
                entry.email,
                entry.password
            ],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn edit(&self, entry: User) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire database lock")?;
        conn.execute(
            "UPDATE Users 
             SET first_name = ?, last_name = ?, phone_number = ?, date_of_birth = ?, email = ?, password = ? 
             WHERE id = ?",
            params![
                entry.first_name,
                entry.last_name,
                entry.phone_number,
                entry.date_of_birth.map(|d| d.format("%Y-%m-%d").to_string()),
                entry.email,
                entry.password,
                entry.id
            ],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_by_id(&self, id: &u32) -> Result<User, String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire database lock")?;
        let mut stmt = conn.prepare(
            "SELECT id, first_name, last_name, phone_number, date_of_birth, email, password 
             FROM Users WHERE id = ?",
        ).map_err(|e| e.to_string())?;

        stmt.query_row([id], |row| {
            Ok(User {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                phone_number: row.get(3)?,
                date_of_birth: row.get::<_, Option<String>>(4)?
                    .map(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").map_err(|e| e.to_string()))
                    .transpose()?,
                email: row.get(5)?,
                password: row.get(6)?,
            })
        }).map_err(|e| e.to_string())
    }

    pub fn get_all(&self) -> Result<Vec<User>, String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire database lock")?;
        let mut stmt = conn.prepare(
            "SELECT id, first_name, last_name, phone_number, date_of_birth, email 
             FROM Users",
        ).map_err(|e| e.to_string())?;

        let users = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                phone_number: row.get(3)?,
                date_of_birth: row.get::<_, Option<String>>(4)?
                    .map(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").map_err(|e| e.to_string()))
                    .transpose()?,
                email: row.get(5)?,
                password: "".to_string(), // Omit password
            })
        }).map_err(|e| e.to_string())?;

        users.collect::<Result<Vec<User>, _>>().map_err(|e| e.to_string())
    }

    pub fn delete(&self, id: &u32) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire database lock")?;
        conn.execute("DELETE FROM Users WHERE id = ?", [id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
