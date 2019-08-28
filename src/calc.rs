// A module to perform various calculations based on inverter and meter readings.
//

extern crate chrono;
use chrono::Date;
use chrono::Local;

/// Readings taken from the meter and inverter.
pub struct Reading {
    pub date : Date<Local>, 
    pub generation: f32, 
    pub grid_import: f32,
    pub grid_export: f32,
}
