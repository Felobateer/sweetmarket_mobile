use std::path::Path;
use rusqlite::{Connection, params};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cart {
    pub id: Option<u32>,
    pub product_id: u32,
    pub user_id: u32,
    pub purchase_date: Option<NaiveDate>,
    pub sold: bool,
}

pub struct Carts {
    conn: Arc<Mutex<Connection>>,
}

impl Carts {
    pub fn init() -> Result<Self, String> {
        let db_path = Path::new("backend/src/database/Carts_data.db");
        let conn = Connection::open(db_path).map_err(|e| format!("Failed to open DB: {}", e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS Carts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                product_id INTEGER NOT NULL,
                user_id INTEGER NOT NULL,
                purchase_date TEXT,
                sold INTEGER DEFAULT 0 NOT NULL,
                FOREIGN KEY (product_id) REFERENCES Products(id),
                FOREIGN KEY (user_id) REFERENCES Users(id)
            )",
            [],
        ).map_err(|e| e.to_string())?;

        Ok(Carts {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn add(&self, entry: Cart) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire database lock")?;
        conn.execute(
            "INSERT INTO Carts (product_id, user_id, purchase_date, sold) VALUES (?, ?, ?, ?)",
            params![
                entry.product_id,
                entry.user_id,
                entry.purchase_date.map(|d| d.format("%Y-%m-%d").to_string()),
                entry.sold as i32
            ],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn edit(&self, entry: Cart) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire database lock")?;
        conn.execute(
            "UPDATE Carts SET product_id = ?, user_id = ?, purchase_date = ?, sold = ? WHERE id = ?",
            params![
                entry.product_id,
                entry.user_id,
                entry.purchase_date.map(|d| d.format("%Y-%m-%d").to_string()),
                entry.sold as i32,
                entry.id
            ],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_by_id(&self, id: &u32) -> Result<Cart, String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire database lock")?;
        let mut stmt = conn.prepare(
            "SELECT id, product_id, user_id, purchase_date, sold FROM Carts WHERE id = ?",
        ).map_err(|e| e.to_string())?;

        let cart = stmt.query_row([id], |row| {
            Ok(Cart {
                id: row.get(0)?,
                product_id: row.get(1)?,
                user_id: row.get(2)?,
                purchase_date: row.get::<_, Option<String>>(3)?
                    .map(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").map_err(|e| e.to_string()))
                    .transpose()?,
                sold: row.get::<_, i32>(4)? != 0,
            })
        }).map_err(|e| e.to_string())?;

        Ok(cart)
    }

    pub fn get_all(&self) -> Result<Vec<Cart>, String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire database lock")?;
        let mut stmt = conn.prepare(
            "SELECT id, product_id, user_id, purchase_date, sold FROM Carts",
        ).map_err(|e| e.to_string())?;

        let carts = stmt.query_map([], |row| {
            Ok(Cart {
                id: row.get(0)?,
                product_id: row.get(1)?,
                user_id: row.get(2)?,
                purchase_date: row.get::<_, Option<String>>(3)?
                    .map(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").map_err(|e| e.to_string()))
                    .transpose()?,
                sold: row.get::<_, i32>(4)? != 0,
            })
        }).map_err(|e| e.to_string())?;

        carts.collect::<Result<Vec<Cart>, _>>().map_err(|e| e.to_string())
    }

    pub fn delete(&self, id: &u32) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire database lock")?;
        conn.execute("DELETE FROM Carts WHERE id = ?", [id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
