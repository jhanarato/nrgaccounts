use chrono::NaiveDate;
use nrgaccounts::calc::calculate;
use nrgaccounts::readings::{ Reading, find_change };
use nrgaccounts::console_input::{ask_for_date};
use std::io::prelude::*;
use std::io;

fn main() {
    print!("Enter a date [YYYY-MM-DD]: ");
    io::stdout().flush().ok().expect("Could not flush!");

    let date = ask_for_date();
    match date {
       Ok(d)=> println!("{}", d.format("%d/%m/%Y")),
       Err(e) => panic!("{}", e),
    }

    let first_reading = Reading {
            date: NaiveDate::from_ymd(2019, 7, 18),
            generation: 76.5,
            exports: 1.0,
            imports: 49.0,
        };

    let second_reading = Reading {
        date: NaiveDate::from_ymd(2019, 9, 17),
        generation: 1371.3,
        exports: 442.0,
        imports: 3548.0, 
    };

    let changes = find_change(first_reading, second_reading); 
    let calculation = calculate(changes);
    
    println!("{}", calculation);
}
