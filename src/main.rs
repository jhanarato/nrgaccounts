use chrono::{ Local, TimeZone };
use nrgaccounts::calc::calculate;
use nrgaccounts::readings::{ Reading, find_change };

fn main() {
    let first_reading = Reading {
            date: Local.ymd(2019, 7, 18),
            generation: 76.5,
            exports: 1.0,
            imports: 49.0,
        };

    let second_reading = Reading {
        date: Local.ymd(2019, 9, 17),
        generation: 1371.3,
        exports: 442.0,
        imports: 3548.0, 
    };

    let changes = find_change(first_reading, second_reading); 
    let calculation = calculate(changes);
    
    println!("{}", calculation);
}
