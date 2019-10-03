use sqlite::{ Connection, Value };
use chrono::{ NaiveDate };

use crate::readings::Reading;

pub struct Database {
   connection : Connection,
}

impl Database {
    pub fn open(file : &str) -> Database {
        let connection = sqlite::open(file).unwrap();
        
        Database {
            connection,
        }
    }

    pub fn create_table(&self) {
        self.connection.execute("
            CREATE TABLE reading (
            date TEXT NOT NULL,
            generation TEXT NOT NULL,
            imports TEXT NOT NULL,
            exports TEXT NOT NULL)")
            .expect("Failed to create table");
    }


    pub fn table_exists(&self, name : &str) -> bool {
        let mut statement = self.connection.prepare(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' and name=?")
            .expect("Failed to check table exists.");
        statement.bind(1, name).expect("Failed to bind table name.");
        statement.next().expect("Failed to get first row.");
        let count = statement.read::<i64>(0).expect("Failed to get count of tables.");
        count == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn create_ok() {
        let db = Database::open(":memory:");
        db.create_table();
    }

    #[test]
    fn table_exists() {
        let db = Database::open(":memory:");
        assert!(!db.table_exists("reading"));
        db.create_table();
        assert!(db.table_exists("reading"));
    }
}

