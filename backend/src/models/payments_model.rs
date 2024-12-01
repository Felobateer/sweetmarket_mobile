use std::path::Path;
use rusqlite::{Connection, params};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CardType {
    Mastercard,
    Visa,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Payment {
    pub id: Option<u32>,
    pub user_id: u32,
    pub card_name: String,
    pub card_number: u32,
    pub expiry_date: String,
    pub card_type: CardType,
    pub cvc: u32,
}

pub struct Payments {
    conn: Arc<Mutex<Connection>>,
}

impl Payments {
    pub fn init() -> Result<Self, String> {
        let db_path = Path::new("backend/src/database/Payments_data.db");
        let conn = Connection::open(db_path).map_err(|e| format!("Failed to open DB: {}", e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS Payments (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                card_name TEXT NOT NULL,
                card_number INTEGER NOT NULL,
                expiry_date TEXT NOT NULL,
                card_type TEXT NOT NULL,
                cvc INTEGER NOT NULL,
                FOREIGN KEY (user_id) REFERENCES Users(id)
            )",
            [],
        ).map_err(|e| e.to_string())?;

        Ok(Payments { conn: Arc::new(Mutex::new(conn)) })
    }

    pub fn add(&self, entry: Payment) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire database lock")?;
        conn.execute(
            "INSERT INTO Payments (user_id, card_name, card_number, expiry_date, card_type, cvc)
             VALUES (?, ?, ?, ?, ?, ?)",
            params![
                entry.user_id,
                entry.card_name,
                entry.card_number,
                entry.expiry_date,
                format!("{:?}", entry.card_type),
                entry.cvc
            ],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn edit(&self, entry: Payment) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire database lock")?;
        conn.execute(
            "UPDATE Payments SET user_id = ?, card_name = ?, card_number = ?, expiry_date = ?, card_type = ?, cvc = ? WHERE id = ?",
            params![
                entry.user_id,
                entry.card_name,
                entry.card_number,
                entry.expiry_date,
                format!("{:?}", entry.card_type),
                entry.cvc,
                entry.id
            ],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_by_id(&self, id: &u32) -> Result<Payment, String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire database lock")?;
        let mut stmt = conn.prepare(
            "SELECT id, user_id, card_name, card_number, expiry_date, card_type, cvc FROM Payments WHERE id = ?",
        ).map_err(|e| e.to_string())?;
        let payment = stmt.query_row([id], |row| {
            Ok(Payment {
                id: row.get(0)?,
                user_id: row.get(1)?,
                card_name: row.get(2)?,
                card_number: row.get(3)?,
                expiry_date: row.get(4)?,
                card_type: match row.get::<_, String>(5)?.as_str() {
                    "Mastercard" => CardType::Mastercard,
                    "Visa" => CardType::Visa,
                    _ => return Err(rusqlite::Error::InvalidQuery),
                },
                cvc: row.get(6)?,
            })
        }).map_err(|e| e.to_string())?;
        Ok(payment)
    }

    pub fn get_all(&self) -> Result<Vec<Payment>, String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire database lock")?;
        let mut stmt = conn.prepare(
            "SELECT id, user_id, card_name, card_number, expiry_date, card_type, cvc FROM Payments",
        ).map_err(|e| e.to_string())?;
        let payments = stmt.query_map([], |row| {
            Ok(Payment {
                id: row.get(0)?,
                user_id: row.get(1)?,
                card_name: row.get(2)?,
                card_number: row.get(3)?,
                expiry_date: row.get(4)?,
                card_type: match row.get::<_, String>(5)?.as_str() {
                    "Mastercard" => CardType::Mastercard,
                    "Visa" => CardType::Visa,
                    _ => return Err(rusqlite::Error::InvalidQuery),
                },
                cvc: row.get(6)?,
            })
        }).map_err(|e| e.to_string())?;

        payments.collect::<Result<Vec<Payment>, _>>().map_err(|e| e.to_string())
    }

    pub fn delete(&self, id: &u32) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|_| "Failed to acquire database lock")?;
        conn.execute("DELETE FROM Payments WHERE id = ?", [id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}