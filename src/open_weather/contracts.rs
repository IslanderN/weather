use crate::contracts::*;
use chrono::{NaiveDateTime};

pub enum Temperature {
    Value(f64),
    Range { min: f64, max: f64}
}

pub struct Weather {
    pub date: NaiveDateTime,
    pub temperature: Temperature
}

impl GetTempeature for Weather {
    fn get_temperature(&self) -> String {
        match &self.temperature {
            Temperature::Value(v) => format!("{v:.1} C"),
            Temperature::Range { min, max } => format!("{min:.1} - {max:.1} C")
        }
    }
}

impl GetDate for Weather {
    fn get_date(&self) -> String {
        self.date.to_string()
    }
}

impl Forecast for Weather {}


pub struct Address {
    pub lat: f64,
    pub lon: f64,
    pub full_address: String
}

impl GetFullAddress for Address {
    fn get_full_address(&self) -> &str {
        &self.full_address
    }
}