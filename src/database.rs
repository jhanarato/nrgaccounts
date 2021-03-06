use sqlite::{ Connection, Value };
use chrono::{ NaiveDate };

use crate::readings::Reading;


// Helper function. Take a row, get a reading.
fn row_to_reading(row : &[Value]) -> Reading {
    let date = row[0].as_string().unwrap();
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    let generation = row[1].as_float().unwrap() as f32;
    let imports =    row[2].as_float().unwrap() as f32;
    let exports =    row[3].as_float().unwrap() as f32;
    Reading { date, generation, imports, exports }
}

// Convert the f32 to f64 and round to one decimal place.
fn convert_for_sqlite(f32_val : f32) -> f64 {
    let f64_val = f32_val as f64;
    (f64_val * 10.0).round() / 10.0
}

pub struct Database {
   connection : Connection,
}

impl Database {
    /// Get a connection to a database with the given filename.
    pub fn open(file : &str) -> Database {
        let connection = sqlite::open(file).unwrap();
        
        let db = Database {
            connection,
        };

        if !db.table_exists("reading") {
            db.create_tables();
        }

        db
    }

    /// Create all needed tables.
    pub fn create_tables(&self) {
        self.connection.execute("
            CREATE TABLE reading (
            date TEXT NOT NULL,
            generation REAL NOT NULL,
            imports REAL NOT NULL,
            exports REAL NOT NULL)").unwrap();
    }

    /// True if a table with the given name exists.
    pub fn table_exists(&self, name : &str) -> bool {
        let mut statement = self.connection.prepare(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' and name=?")
            .expect("Failed to check table exists.");
        statement.bind(1, name).unwrap();
        statement.next().unwrap();
        let count = statement.read::<i64>(0).unwrap();
        count == 1
    }
    
    /// Add a new reading to the database.
    pub fn add_reading(&self, reading : &Reading) {
        let mut cursor = self.connection.prepare(
            "INSERT INTO reading ( date, generation, imports, exports )
             VALUES ( ?, ?, ?, ? )").unwrap().cursor();
        
        let date = reading.date.format("%Y-%m-%d").to_string();
        let generation = convert_for_sqlite(reading.generation);
        let imports = convert_for_sqlite(reading.imports);
        let exports = convert_for_sqlite(reading.exports);
        cursor.bind(&[Value::String(date), 
                      Value::Float(generation),
                      Value::Float(imports),
                      Value::Float(exports)]).unwrap();
        
        cursor.next().unwrap();
    }
    

    /// Get the reading for a given date or none if it doesn't exists.
    pub fn get_reading_for_date(&self, date : NaiveDate) -> Option<Reading> {
        let mut cursor = self.connection.prepare(
            "SELECT * FROM reading WHERE date = ?")
            .unwrap().cursor();

        let date = date.format("%Y-%m-%d").to_string();

        cursor.bind(&[Value::String(date)]).unwrap();

        let first_row = cursor.next().unwrap();
        match first_row {
            Some(row) => Some(row_to_reading(&row)),
            None => None,
        }
    }

    pub fn most_recent_reading(&self) -> Option<Reading> {
        let mut cursor = self.connection.prepare(
            "SELECT * FROM reading 
             ORDER BY date DESC
             LIMIT 1").unwrap().cursor();
        
        let first_row = cursor.next().unwrap();
        match first_row {
            Some(row) => Some(row_to_reading(&row)),
            None => None,
        }
    }
    
    pub fn number_of_readings(&self) -> i64 {
        let mut cursor = self.connection.prepare(
            "SELECT COUNT(*) FROM reading").unwrap().cursor();
        
        let row = cursor.next().unwrap().unwrap();
        row[0].as_integer().unwrap()   
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn row_to_reading_ok() {
        let row = [Value::String("2010-10-10".to_string()), 
                   Value::Float(10.0),
                   Value::Float(20.0),
                   Value::Float(5.5)];

        let reading = row_to_reading(&row);

        assert_eq!(reading.date, NaiveDate::from_ymd(2010, 10, 10));
        assert_eq!(reading.generation, 10.0);
        assert_eq!(reading.imports, 20.0);
        assert_eq!(reading.exports, 5.5);
    }

    #[test]
    fn table_exists() {
        let db = Database::open(":memory:");
        assert!(db.table_exists("reading"));
    }

    #[test]
    fn add_retrieve_reading() {
        let db = Database::open(":memory:");

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

    #[test]
    fn most_recent_reading_ok() {
        let db = Database::open(":memory:");

        let reading_1 = Reading {
                date : NaiveDate::from_ymd(2019, 10, 10),
                generation : 30.0,
                imports : 20.0,
                exports : 5.0,
            };

        // Most recent reading.
        let reading_2 = Reading {
                date : NaiveDate::from_ymd(2019, 10, 12),
                generation : 29.0,
                imports : 19.0,
                exports : 4.0,
            };

        let reading_3 = Reading {
                date : NaiveDate::from_ymd(2019, 10, 11),
                generation : 28.0,
                imports : 18.0,
                exports : 3.0,
            
            };

        db.add_reading(&reading_1);
        db.add_reading(&reading_2);
        db.add_reading(&reading_3);

        let most_recent = db.most_recent_reading().unwrap();

        assert_eq!(most_recent.date, reading_2.date);
        assert_eq!(most_recent.generation, reading_2.generation);
        assert_eq!(most_recent.imports, reading_2.imports);
        assert_eq!(most_recent.exports, reading_2.exports);

    }        
    
    #[test]
    fn number_of_readings_ok() {
        let db = Database::open(":memory:");
        
        let reading_1 = Reading {
                date : NaiveDate::from_ymd(2019, 10, 10),
                generation : 30.0,
                imports : 20.0,
                exports : 5.0,
            };

        let reading_2 = Reading {
                date : NaiveDate::from_ymd(2019, 10, 11),
                generation : 60.0,
                imports : 30.0,
                exports : 10.0,
            };

        let reading_3 = Reading {
                date : NaiveDate::from_ymd(2019, 10, 12),
                generation : 90.0,
                imports : 60.0,
                exports : 20.0,
            };

        assert_eq!(db.number_of_readings(), 0);
        db.add_reading(&reading_1);
        assert_eq!(db.number_of_readings(), 1);
        db.add_reading(&reading_2);
        assert_eq!(db.number_of_readings(), 2);
        db.add_reading(&reading_3);
        assert_eq!(db.number_of_readings(), 3);
    }

    #[test]
    fn convert_for_sqlite_ok() {
        let in_f32 : f32 = 1.1;
        let out_f64 : f64 = convert_for_sqlite(in_f32);
        assert_eq!(out_f64, 1.1);
    }
}


