use async_trait::async_trait;
use chrono::{NaiveDate};

pub trait GetTempeature {
    fn get_temperature(&self) -> String;
}

pub trait GetDate {
    fn get_date(&self) -> String;
    
}

pub trait Forecast: GetDate + GetTempeature {}

pub trait GetFullAddress {
    fn get_full_address(&self) -> &str;
}


#[async_trait]
pub trait GetAddress {
    type Address: GetFullAddress;
    async fn find_addresses(&mut self, address: String) -> core::result::Result<&Vec<Self::Address>, String>;
    fn choose_address(&mut self, index: usize) -> core::result::Result<(), String>;
}


#[async_trait]
pub trait GetWeather {
    type Weather: Forecast;
    async fn get_weather(&self, date: Option<NaiveDate>) -> core::result::Result<Self::Weather, String>;
}