/// Calculate stats based on meter readings.
use std::fmt;

use crate::readings::DiurnalChange;

/// The tarff to put a kilowatt/hour of energy into the grid.
pub const FEED_IN_TARIFF : f32 = 7.135;

/// The tariff for consuming a kilowatt/hour of energy from the grid.
pub const SUPPLY_TARIFF : f32 =  25.752;

/// Calculate a variety of values related to energy consumption
/// and production and return the information as a Calculation.
pub fn calculate(change: DiurnalChange) -> Calculation {

    let self_consumption_kwh = change.generation - change.exports;
    let total_consumption_kwh = self_consumption_kwh + change.imports;
    
    let self_consumption = SelfConsumption {
        kwh : self_consumption_kwh,
        fraction_of_generation : self_consumption_kwh / change.generation,
        fraction_of_total_use : self_consumption_kwh / total_consumption_kwh,
    };
    let from_self_consumption = self_consumption.kwh * SUPPLY_TARIFF;
    let from_exports = change.exports * FEED_IN_TARIFF;

    let savings = Savings {
        from_exports,
        from_self_consumption,
        total : from_exports + from_self_consumption,
    };

    let calculation = Calculation {
        generation_kwh: change.generation,
        grid_import_kwh: change.imports,
        grid_export_kwh: change.exports,
        total_consumption_kwh,
        self_consumption,
        savings,
    };
    calculation
}

/// A simple data structure for storing a variety of information
/// about energy consumption and production.
pub struct Calculation {
    /// The amount of generated energy.
    pub generation_kwh: f32,
    /// The amount of energy imported from the grid.
    pub grid_import_kwh: f32,
    /// The amount of energy exported to the grid.
    pub grid_export_kwh: f32,
    /// The total amount of energy consumed.
    pub total_consumption_kwh: f32,

    /// Information related to self consumption.
    pub self_consumption: SelfConsumption,
    /// The amount of money saved.
    pub savings: Savings,
}

/// Information related to energy produced and consumed directly.
pub struct SelfConsumption {
    /// The amount of energy in kilowatt/hours.
    pub kwh: f32,
    /// A number between 0 and 1 representing the self consumption
    /// as a fraction of the total energy used for the period.
    pub fraction_of_total_use: f32,
    /// A number between 0 and 1 representing self consumption as
    /// a fraction of what has been generated on-site.
    pub fraction_of_generation: f32,
}

/// How much money we have saved during this period.
pub struct Savings {
    /// The amount saved due to energy self-consumed.
    pub from_self_consumption: f32,
    /// The amount earned by exporting energy to the grid.
    pub from_exports: f32,
    /// Overall savings due to solar.
    pub total: f32,
}

/// Allow a Calculation object to be passed to println!() etc.
impl fmt::Display for Calculation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines: Vec<String> = Vec::new();
        lines.push(format!("Generation: {:.2} kWh/day\n", self.generation_kwh));
        lines.push(format!("Exports:    {:.2} kWh/day\n", self.grid_export_kwh));
        lines.push(format!("Imports:    {:.2} kWh/day\n", self.grid_import_kwh));
        lines.push(format!("Total use:  {:.2} kWh/day\n", self.total_consumption_kwh));

        lines.push("Self consumption:\n".to_string());
        lines.push(format!("    Energy:         {:.2} kWh/day\n", self.self_consumption.kwh));
        lines.push(format!("    % of total:     {:.2}%\n", 
                           (self.self_consumption.fraction_of_total_use * 100.0)));
        lines.push(format!("    % of generated: {:.2}%\n", 
                           (self.self_consumption.fraction_of_generation * 100.0)));
         
        lines.push("Savings:\n".to_string());
        lines.push(format!("   By self-consumption: ${:.2}\n", 
                           self.savings.from_self_consumption));
        lines.push(format!("   From exports:        ${:.2}\n", self.savings.from_exports));
        lines.push(format!("   Total:               ${:.2}\n", self.savings.total));

        
        let mut output = String::new();

        for line in lines.iter() {
            output.push_str(line.as_str());
        }
        
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_kwh() {
        let change = DiurnalChange {
            generation: 3.0, 
            imports: 2.0,
            exports: 2.0,
        };

        let expected = change.generation;
        let calculation = calculate(change);
        assert_eq!(calculation.generation_kwh, expected);
    }

    #[test]
    fn grid_import_kwh() {
        let change = DiurnalChange {
            generation: 3.0, 
            imports: 1.0,
            exports: 2.0,
        };

        let expected = change.imports;
        let calculation = calculate(change);
        assert_eq!(calculation.grid_import_kwh, expected);
    }

    #[test]
    fn grid_export_kwh() {
        let change = DiurnalChange {
            generation: 3.0, 
            imports: 1.0,
            exports: 2.0,
        };
        let expected = change.exports;
        let calculation = calculate(change);
        assert_eq!(calculation.grid_export_kwh, expected);
    }

    #[test]
    fn total_consumption_kwh() {
        let change = DiurnalChange {
            generation: 3.0, 
            imports: 2.0,
            exports: 1.0,
        };
        let expected = 4.0;
        let calculation = calculate(change);
        assert_eq!(calculation.total_consumption_kwh, expected);
    }

    #[test]
    fn self_consumption_kwh() {
        let change = DiurnalChange {
            generation: 3.0, 
            imports: 2.0,
            exports: 1.0,
        };
        let expected = 2.0;
        let calculation = calculate(change);
        assert_eq!(calculation.self_consumption.kwh, expected);
    }

    #[test]
    fn fraction_of_total_use() {
        let change = DiurnalChange {
            generation: 3.0, 
            imports: 2.0,
            exports: 1.0,
        };
        let expected = 0.5;
        let calculation = calculate(change);
        assert_eq!(calculation.self_consumption.fraction_of_total_use, expected);
    }
    
    #[test]
    fn fraction_of_generation() {
        let change = DiurnalChange {
            generation: 10.0, 
            imports: 4.0,
            exports: 2.5,
        };
        let expected = 0.75;
        let calculation = calculate(change);
        assert_eq!(calculation.self_consumption.fraction_of_generation, expected);
    }
    
    #[test]
    fn savings_from_self_consumption() {
        let change = DiurnalChange {
            generation: 3.0, 
            imports: 2.0,
            exports: 1.0,
        };
        let expected = 2.0 * SUPPLY_TARIFF;
        let calculation = calculate(change);
        assert_eq!(calculation.savings.from_self_consumption, expected);
    }

    #[test]
    fn savings_from_exports() {
        let change = DiurnalChange {
            generation: 3.0, 
            imports: 2.0,
            exports: 3.0,
        };
        let expected = 3.0 * FEED_IN_TARIFF;
        let calculation = calculate(change);
        assert_eq!(calculation.savings.from_exports, expected);
    }
  
    #[test]
    fn savings_total() {
        let change = DiurnalChange {
            generation: 7.0, 
            imports: 2.0,
            exports: 3.0,
        };
        let expected = 4.0 * SUPPLY_TARIFF + 3.0 * FEED_IN_TARIFF;
        let calculation = calculate(change);
        assert_eq!(calculation.savings.total, expected);
    }

}
