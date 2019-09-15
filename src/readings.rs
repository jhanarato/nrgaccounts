use chrono::{ Local, Date, TimeZone };

/// A collection of readings for a given date. 
pub struct Reading {
    /// The date the readings were made.
    pub date: Date<Local>,
    /// The total generated energy from the inverter in kilowatt / hours.
    pub generation: f32,
    /// The total amount of energy exported to the grid 
    /// from the electricity meter in kilowatt / hours.
    pub exports: f32,
    /// The total amount of energy imported from 
    /// the grid by the electricity meter in kilowatt / hours.. 
    pub imports: f32,
}

/// Amounts of energy from dusk on one day to dusk
/// on the next. May be an average depending on how
/// often you read the meter.
pub struct DiurnalChange {
    /// The inverter generation in kilowatt / hours.
    pub generation: f32,
    /// The energy exported to the grid in kilowatt / hours.
    pub exports: f32,
    /// The energy imported from the grid in kilowatt /hours.
    pub imports: f32,
}

/// Given two readings on different days, calculate the 
/// change in the values per day.
pub fn find_change(first: Reading, second: Reading) -> DiurnalChange {
    let duration = second.date.signed_duration_since(first.date);
    let days_spanned = duration.num_days();
    let days_spanned = days_spanned as f32; 

    // Get the difference.
    let generation = second.generation - first.generation;
    let exports = second.exports - first.exports;
    let imports = second.imports - first.imports;

    // Get the averages
    let generation = generation / days_spanned;
    let exports = exports / days_spanned;
    let imports = imports / days_spanned;

    DiurnalChange {
        generation,
        exports,
        imports, 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_change() {
        let first_reading = Reading {
            date: Local.ymd(2001, 1, 1),
            generation: 10.0,
            exports: 7.0,
            imports: 2.0,
        };

        let second_reading = Reading {
            date: Local.ymd(2001, 1, 5),
            generation: 30.0,
            exports: 19.0,
            imports: 6.0, 
        };

        let change = find_change(first_reading, second_reading);
        assert_eq!(change.generation, 5.0);
        assert_eq!(change.exports, 3.0);
        assert_eq!(change.imports, 1.0);
    }
}
