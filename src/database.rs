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
            generation REAL NOT NULL,
            imports REAL NOT NULL,
            exports REAL NOT NULL)").unwrap();
    }


    pub fn table_exists(&self, name : &str) -> bool {
        let mut statement = self.connection.prepare(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' and name=?")
            .expect("Failed to check table exists.");
        statement.bind(1, name).unwrap();
        statement.next().unwrap();
        let count = statement.read::<i64>(0).unwrap();
        count == 1
    }

    pub fn add_reading(&self, reading : &Reading) {
        let mut cursor = self.connection.prepare(
            "INSERT INTO reading ( date, generation, imports, exports )
             VALUES ( ?, ?, ?, ? )").unwrap().cursor();
        
        let date = reading.date.format("%Y-%m-%d").to_string();
       
        cursor.bind(&[Value::String(date), 
                      Value::Float(reading.generation as f64),
                      Value::Float(reading.imports as f64),
                      Value::Float(reading.exports as f64)]).unwrap();
        
        cursor.next().unwrap();
    }

    pub fn get_reading_for_date(&self, date : NaiveDate) -> Option<Reading> {
        let mut cursor = self.connection.prepare(
            "SELECT * FROM reading WHERE date = ?")
            .unwrap().cursor();

        let date = date.format("%Y-%m-%d").to_string();

        cursor.bind(&[Value::String(date)]).unwrap();

        let first_row = cursor.next().unwrap();
        match first_row {
            Some(row) => {
                let date = row[0].as_string().unwrap();
                let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
                let generation = row[1].as_float().unwrap() as f32;
                let imports =    row[2].as_float().unwrap() as f32;
                let exports =    row[3].as_float().unwrap() as f32;
                Some(Reading { date, generation, imports, exports })
            },
            None => None,
        }
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

    #[test]
    fn add_retrieve_reading() {
        let db = Database::open(":memory:");
        db.create_table();

        let reading_in = Reading {
                date : NaiveDate::from_ymd(2019, 10, 4),
                generation : 3.0,
                imports : 5.0,
                exports : 1.0,
            };

        match db.get_reading_for_date(reading_in.date) {
            None => (),
            _ => panic!("Unexpected got a result."),
        }


        db.add_reading(&reading_in);
    
        let reading_out = match db.get_reading_for_date(reading_in.date) {
            Some(reading) => reading, 
            None => panic!("No reading returned for date {}.", 
                           reading_in.date.format("%Y-%m-%d")),
        };

        assert_eq!(reading_in.date, reading_out.date);
        assert_eq!(reading_in.generation, reading_out.generation);
        assert_eq!(reading_in.imports, reading_out.imports);
        assert_eq!(reading_in.exports, reading_out.exports);
    }
}


