extern crate chrono;

use chrono::{Local, TimeZone};
use crate::calc::Reading;
pub mod calc;

fn main() {
    let reading = Reading {
                    date: Local.ymd(2001, 1, 1),
                    generation: 10.0,
                    grid_import: 10.0,
                    grid_export: 10.0,
    };

    println!("Generated {}", reading.generation);
                    
     
}
