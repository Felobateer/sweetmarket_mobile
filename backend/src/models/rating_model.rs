use std::path::Path;
use rusqlite::{Connection, params};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rate {
    pub id: Option<u32>,
    pub product_id: u32,
    pub user_id: u32,
    pub rate: u32,
    pub comment: Option<String>,
}

pub struct Rates {
    conn: Arc<Mutex<Connection>>,
}

impl Rates {
    pub fn init() -> Result<Self, String> {
        let db_path = Path::new("backend/src/database/Rates_data.db");
        let conn = Connection::open(db_path).map_err(|e| format!("Failed to open DB: {}", e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS Rates (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id REFERENCE KEY Products,
            user_id REFERENCE KEY Users,
            rate INTEGER NOT NULL,
            comment TEXT NULL,
            FOREIGN KEY (product_id) REFERENCES Products(id),
            FOREIGN KEY (user_id) REFERENCES Users(id)
        )",
        [],
        ).map_err(|e| e.to_string())?;

        Ok(Rates {conn: Arc::new(Mutex::new(conn))})
    }

    pub fn add(&self, entry: Rate) -> Result<(), String> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        tx.execute("INSERT INTO Rates (product_id, user_id, rate, comment) VALUES (?, ?, ?, ?)", params![entry.product_id, entry.user_id, entry.rate, entry.comment]).map_err(|e| e.to_string())?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn edit(&self, entry: Rate) -> Result<(), String> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        tx.execute("UPDATE Rates SET product_id = ?, user_id = ?, rate = ?, comment = ? WHERE id = ?", params![entry.product_id, entry.user_id, entry.rate, entry.comment , entry.id]).map_err(|e| e.to_string())?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_by_id(&self, id: &u32) -> Result<Rate, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, product_id, user_id, rate, comment FROM Rates WHERE id = ?").map_err(|e| e.to_string())?;
        let Rate = stmt.query_row([id], |row| {
            Ok(Rate {
                id: row.get(0)?,
                product_id: row.get(1)?,
                user_id: row.get(2)?,
                rate: row.get(3)?,
                comment: row.get(4)?,
            })
        }).map_err(|e| e.to_string())?;
        Ok(Rate)
    }
    
    pub fn get_all(&self) -> Result<Vec<Rate>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, product_id, user_id, rate, comment FROM Rates").map_err(|e| e.to_string())?;
        let Rates = stmt.query_map([], |row| {
            Ok(Rate {
                id: row.get(0)?,
                product_id: row.get(1)?,
                user_id: row.get(2)?,
                rate: row.get(3)?,
                comment: row.get(4)?,
            })
        }).map_err(|e| e.to_string())?;

        let Rates = Rates.collect::<Result<Vec<Rate>, _>>().map_err(|e| e.to_string())?;
        Ok(Rates)
    }

    pub fn delete(&self, id: &u32) -> Result<(), String> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        tx.execute("DELETE FROM Rates WHERE id = ?", [id]).map_err(|e| e.to_string())?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }
}