use chrono::{ NaiveDate };

/// A collection of readings for a given date. 
pub struct Reading {
    /// The date the readings were made.
    pub date: NaiveDate,
    /// The total generated energy from the inverter in kilowatt / hours.
    pub generation: f32,
    /// The total amount of energy exported to the grid 
    /// from the electricity meter in kilowatt / hours.
    pub exports: f32,
    /// The total amount of energy imported from 
    /// the grid by the electricity meter in kilowatt / hours.. 
    pub imports: f32,
}

/// Two readings, the first being earlier than the second.
pub struct ReadingPair {
    pub first: Reading,
    pub second: Reading,
}

impl ReadingPair {
    pub fn days_spanned(&self) -> f32 {
        let duration = self.second.date.signed_duration_since(self.first.date);
        let days_spanned = duration.num_days();
        days_spanned as f32
    }
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
pub fn find_change(pair: &ReadingPair) -> DiurnalChange {

    // Get the difference.
    let generation = pair.second.generation - pair.first.generation;
    let exports = pair.second.exports - pair.first.exports;
    let imports = pair.second.imports - pair.first.imports;

    // Get the averages
    let generation = generation / pair.days_spanned();
    let exports = exports / pair.days_spanned();
    let imports = imports / pair.days_spanned();

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
        let first = Reading {
            date: NaiveDate::from_ymd(2001, 1, 1),
            generation: 10.0,
            exports: 7.0,
            imports: 2.0,
        };

        let second = Reading {
            date: NaiveDate::from_ymd(2001, 1, 5),
            generation: 30.0,
            exports: 19.0,
            imports: 6.0, 
        };

        let pair = ReadingPair { first, second };

        let change = find_change(&pair);
        assert_eq!(change.generation, 5.0);
        assert_eq!(change.exports, 3.0);
        assert_eq!(change.imports, 1.0);
    }
}
