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
        let conn = Connection::open(db_path).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS Users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            firstName TEXT NOT NULL,
            lastName TEXT NOT NULL,
            phoneNumber INTEGER NOT NULL,
            dateOfBirth TEXT NULL,
            email TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        [],
        ).map_err(|e| e.to_string())?;

        Ok(Users {conn: Arc::new(Mutex::new(conn))})
    }

    pub fn add(&self, entry: User) -> Result<(), String> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        tx.execute("INSERT INTO Users (firstName, lastName, phoneNumber, dateOfBirth, email, password) VALUES (?, ?, ?, ?, ?, ?)", params![entry.first_name, entry.last_name, entry.phone_number, entry.date_of_birth, entry.email, entry.password]).map_err(|e| e.to_string())?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn edit(&self, entry: User) -> Result<(), String> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        tx.execute("UPDATE Users SET firstName = ?, lastName = ?, phoneNumber = ?, dateOfBirth = ?, email = ?, password = ? WHERE id = ?", params![entry.first_name, entry.last_name, entry.phone_number, entry.date_of_birth, entry.email, entry.password, entry.id]).map_err(|e| e.to_string())?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_by_id(&self, id: &u32) -> Result<User, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, firstName, lastName, phoneNumber, dateOfBirth, email, password FROM Users WHERE id = ?").map_err(|e| e.to_string())?;
        let User = stmt.query_row([id], |row| {
            Ok(User {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                phone_number: row.get(3)?,
                date_of_birth: row.get(4)?,
                email: row.get(5)?,
                password: "".to_string(),
            })
        }).map_err(|e| e.to_string())?;
        Ok(User)
    }
    
    pub fn get_all(&self) -> Result<Vec<User>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, firstName, lastName, phoneNumber, dateOfBirth, email FROM Users").map_err(|e| e.to_string())?;
        let Users = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                phone_number: row.get(3)?,
                date_of_birth: row.get(4)?,
                email: row.get(5)?,
            })
        }).map_err(|e| e.to_string())?;

        let Users = Users.collect::<Result<Vec<User>, _>>().map_err(|e| e.to_string())?;
        Ok(Users)
    }

    pub fn delete(&self, id: &u32) -> Result<(), String> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        tx.execute("DELETE FROM Users WHERE id = ?", [id]).map_err(|e| e.to_string())?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }
}