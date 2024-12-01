use std::path::Path;
use rusqlite::{Connection, params};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Product {
    pub id: Option<u32>,
    pub name: String,
    pub description: String,
    pub img: String,
    pub store: String,
    pub price: u32,
    pub rating: Option<u32>,
    pub rate_number: Option<u32>,
    pub discount: Option<u32>,
}

pub struct Products {
    conn: Arc<Mutex<Connection>>,
}

impl Products {
    pub fn init() -> Result<Self, String> {
        let db_path = Path::new("backend/src/database/Products_data.db");
        let conn = Connection::open(db_path).map_err(|e| format!("Failed to open DB: {}", e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS Products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            img TEXT NOT NULL,
            store TEXT NOT NULL,
            price INTEGER NOT NULL,
            rating INTEGER NULL,
            rate_number INTEGER NULL,
            discount INTEGER NULL,
        )",
        [],
        ).map_err(|e| e.to_string())?;

        Ok(Products {conn: Arc::new(Mutex::new(conn))})
    }

    pub fn add(&self, entry: Product) -> Result<(), String> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        tx.execute("INSERT INTO Products (name, description, img, store, price, rating, rate_number, discount) VALUES (?, ?, ?, ?, ?, ?, ?, ?)", params![entry.name, entry.description, entry.img, entry.store, entry.price, entry.rating, entry.rate_number, entry.discount]).map_err(|e| e.to_string())?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn edit(&self, entry: Product) -> Result<(), String> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        tx.execute("UPDATE Products SET name = ?, description = ?, img = ?, store = ?, price = ?, rating = ?, rate_number = ?, discount = ? WHERE id = ?", params![entry.first_name, entry.last_name, entry.phone_number, entry.date_of_birth, entry.email, entry.password, entry.discount, entry.id]).map_err(|e| e.to_string())?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_by_id(&self, id: &u32) -> Result<Product, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, description, img, store, price, rating, rate_number, discount FROM Products WHERE id = ?").map_err(|e| e.to_string())?;
        let Product = stmt.query_row([id], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                img: row.get(3)?,
                store: row.get(4)?,
                price: row.get(5)?,
                rating: row.get(6)?,
                rate_number: row.get(7)?,
                discount: row.get(8)?,
            })
        }).map_err(|e| e.to_string())?;
        Ok(Product)
    }
    
    pub fn get_all(&self) -> Result<Vec<Product>, String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, description, img, store, price, rating, rate_number, discount FROM Products").map_err(|e| e.to_string())?;
        let Products = stmt.query_map([], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                img: row.get(3)?,
                store: row.get(4)?,
                price: row.get(5)?,
                rating: row.get(6),
                rate_number: row.get(7)?,
                discount: row.get(8)?,
            })
        }).map_err(|e| e.to_string())?;

        let Products = Products.collect::<Result<Vec<Product>, _>>().map_err(|e| e.to_string())?;
        Ok(Products)
    }

    pub fn delete(&self, id: &u32) -> Result<(), String> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        tx.execute("DELETE FROM Products WHERE id = ?", [id]).map_err(|e| e.to_string())?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }
}