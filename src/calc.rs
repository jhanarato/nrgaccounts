use chrono::Date;
use chrono::Local;

pub struct Reading {
    pub date: Date<Local>, 
    pub generation: f32, 
    pub grid_import: f32,
    pub grid_export: f32,
}

// A set of calculations for a given period of time.
pub struct Calculation {
    pub days: u32,         // Number of days between readings.
    pub generation: f32,    
    pub grid_import: f32,  
    pub grid_export: f32,
}

impl Calculation {
    // Get the amount of energy delivered by the inverter
    // used on the premises in kilowatt/hours.
    pub fn self_consumption(&self) -> f32 {
        self.generation - self.grid_export
    }

    // Get a value between 0 and 1 for self generation
    // as a portion of total generation.
    pub fn percentage_of_generation_self_consumed(&self) -> f32 {
        self.self_consumption() / self.generation * 100.0
    }
}

// Beginning of unit test section.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn self_consumption() {
        let calc = Calculation {
            days: 1,
            generation:  10.0,
            grid_import: 50.0, // Not important for this test.
            grid_export: 7.0,
        };

        assert_eq!(calc.self_consumption(), 3.0);
    }

    #[test]
    fn percentage_of_generation_self_consumed() {
        let calc = Calculation {
            days: 1,
            generation:  30.0,
            grid_import: 50.0, // Not important for this test.
            grid_export: 3.0,
        };

        assert_eq!(calc.percentage_of_generation_self_consumed(), 90.0);
    }
}
