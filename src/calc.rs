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

// Self consumption fractions represented as a 
// number between 0 and 1.
pub struct Fractions {
    pub of_generation: f32,
    pub of_total_consumption: f32,
}

impl Calculation {
    // Get the amount of energy delivered by the inverter
    // used on the premises in kilowatt/hours.
    pub fn self_consumption(&self) -> f32 {
        self.generation - self.grid_export
    }
    
    // Get the total amount of energy consumed both
    // from the grid and from the inverter in kilowatt/hours.
    pub fn total_consumption(&self) -> f32 {
        self.self_consumption() + self.grid_import
    }

    pub fn self_consumption_fractions(&self) -> Fractions {
        Fractions {
            of_generation: self.self_consumption() / self.generation,
            of_total_consumption: self.self_consumption() / self.total_consumption(),
        }

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

        assert_eq!(calc.self_consumption_fractions().of_generation, 0.9);
    }

    #[test]
    fn total_consumption () {
        let calc = Calculation {
            days: 1,
            generation:  30.0,
            grid_import: 50.0, 
            grid_export: 3.0, 
        };
        
        assert_eq!(calc.total_consumption(), 77.0)

    }

    #[test]
    fn percentage_of_total_consumption_self_generated() {
        let calc = Calculation {
            days: 1,
            generation:  30.0,
            grid_import: 80.0, 
            grid_export: 10.0, 
        };
        
        assert_eq!(calc.self_consumption_fractions().of_total_consumption, 0.2);
    }
}
