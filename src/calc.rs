use chrono::Date;
use chrono::Local;

/// Readings taken from the meter and inverter.
pub struct Reading {
    pub date : Date<Local>, 
    pub generation: f32, 
    pub grid_import: f32,
    pub grid_export: f32,
}

// A set of calculations for a given period of time.
pub struct Calculation {
    result: String,
}

// Given two readings calculate the stats.
pub fn calculate(first: Reading, second: Reading) -> Calculation {
    Calculation {
        result : String::from("We have a result."),
    }
}

// Beginning of unit test section.
#[cfg(test)]
mod tests {
    use chrono::{Local, TimeZone};
    use super::*;

    #[test]
    fn calculate_returns_something() {
        let first = Reading {
            date: Local.ymd(2001, 1, 1),
            generation: 10.0,
            grid_import: 10.0,
            grid_export: 10.0,
        };
        let second = Reading {
            date: Local.ymd(2001, 1, 1),
            generation: 5.0, 
            grid_import: 5.0,
            grid_export: 5.0,
        };
        let calculation = calculate(first, second);
        assert_eq!(calculation.result, "We have a result.");
    }
}
