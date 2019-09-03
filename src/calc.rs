// Calculate stats based on meter readings.

const FEED_IN_TARIFF : f32 = 7.135;
const SUPPLY_TARIFF : f32 =  25.752;

pub fn calculate(generation_kwh: f32, 
                 grid_import_kwh: f32, 
                 grid_export_kwh: f32) -> Calculation {

    let self_consumption_kwh = generation_kwh - grid_export_kwh;
    let total_consumption_kwh = self_consumption_kwh + grid_import_kwh;
    
    let self_consumption = SelfConsumption {
        kwh : self_consumption_kwh,
        fraction_of_generation : self_consumption_kwh / generation_kwh,
        fraction_of_total_use : self_consumption_kwh / total_consumption_kwh,
    };
    let from_self_consumption = self_consumption.kwh * SUPPLY_TARIFF;
    let from_exports = grid_export_kwh * FEED_IN_TARIFF;

    let savings = Savings {
        from_exports,
        from_self_consumption,
        total : from_exports + from_self_consumption,
    };

    let calculation = Calculation {
        generation_kwh,
        grid_import_kwh,
        grid_export_kwh,
        total_consumption_kwh,
        self_consumption,
        savings,
    };
    calculation
}

pub struct Calculation {
    pub generation_kwh: f32,
    pub grid_import_kwh: f32,
    pub grid_export_kwh: f32,
    pub total_consumption_kwh: f32,

    pub self_consumption: SelfConsumption,
    pub savings: Savings,
}

pub struct SelfConsumption {
    pub kwh: f32,
    pub fraction_of_total_use: f32,
    pub fraction_of_generation: f32,
}

pub struct Savings {
    pub from_self_consumption: f32,
    pub from_exports: f32,
    pub total: f32,
}

