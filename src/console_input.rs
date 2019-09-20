use chrono::{ NaiveDate };
use std::io;
use std::io::prelude::*;
use std::num::ParseFloatError;
use crate::readings::{ Reading, ReadingPair };


/// Using user input from the console get two
/// readings, the earlier reading first.
pub fn get_reading_pair() -> ReadingPair {
    println!("First reading:");
    let first = get_reading();
    println!("Second reading:");
    let second = get_reading();
    ReadingPair { first, second }
}

fn get_reading()-> Reading {

    let date = loop {
        print!("    Date [DD-MM-YY]:    ");
        io::stdout().flush().ok().expect("Could not flush!");
        let date = ask_for_date();
        match date {
           Ok(d)=> break d, 
           Err(_e) => println!("Invalid date."), 
        }
    };

    let generation = loop {
        print!("    Generation [kWh]:   ");
        io::stdout().flush().ok().expect("Could not flush!");
        let generation = ask_for_number();
        match generation {
            Ok(g) => break g,
            Err(_e) => println!("Invalid number."),
        }
    };

    let exports = loop {
        print!("    Grid exports [kWh]: ");
        io::stdout().flush().ok().expect("Could not flush!");
        let exports = ask_for_number();
        match exports {
            Ok(ex) => break ex,
            Err(_e) => println!("Could not parse number, try again."),
        }
    };

    let imports = loop {
        print!("    Grid imports [kWh]: ");
        io::stdout().flush().ok().expect("Could not flush!");
        let imports= ask_for_number();
        match imports {
            Ok(i) => break i,
            Err(_e) => println!("Could not parse number, try again."),
        }
    };

    Reading {
        date,
        generation,
        exports,
        imports,
    }
}

fn ask_for_number() -> Result<f32, ParseFloatError> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.pop() // Remove trailing newline.
        },
        Err(e) => panic!("Unexpected error {}", e),
    };
    input.parse::<f32>() 
}

fn ask_for_date() -> Result<NaiveDate, &'static str> {
    
    let mut input = String::new(); 
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.pop(); // Remove trailing newline.
            parse_date(input.as_str())
        },
        Err(e) => panic!("Unexpected error {}", e),
    }
}

/// Given a date as an ISO formatted string return
/// a chrono date object. Assuming input is valid.
fn parse_date(date_str: &str) -> Result<NaiveDate, &'static str> {
    let date = NaiveDate::parse_from_str(date_str, "%d-%m-%y");
    match date {
        Err(_e) => Err("Failed to parse date"),
        Ok(d) => Ok(d), 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_date_input() {
        let actual_result = parse_date("31-12-99");
        let expected_result = Ok(NaiveDate::from_ymd(1999, 12, 31));
        assert_eq!(actual_result, expected_result); 
    }

    #[test]
    fn bad_date_input() {
        let actual_result = parse_date("*31-12-99");
        let expected_result = Err("Failed to parse date");
        assert_eq!(actual_result, expected_result); 
    }
}

