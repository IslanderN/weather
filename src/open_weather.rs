mod configuration;
pub mod location;
pub mod current_weather;
pub mod future_weather;
mod contracts;

use async_trait::async_trait;
use chrono::{NaiveDate};
use crate::contracts::*;
use crate::location_manager::LocationManager;

use self::contracts::Address;

pub struct OpenWeatherProvider{
    location_manager: LocationManager<Address>
}

impl OpenWeatherProvider {
    pub fn new() -> Self {
        OpenWeatherProvider{location_manager: LocationManager::new()}
    }
}

#[async_trait]
impl GetAddress for OpenWeatherProvider {
    type Address = self::contracts::Address;

    async fn find_addresses(&mut self, address: String) -> core::result::Result<&Vec<Self::Address>, String>{
        location::get_location(address).await
        .map(|addressess| {self.location_manager.store_addresses(addressess)})
        .map(|_| {self.location_manager.get_all_addresses()})

    }

    fn choose_address(&mut self, index: usize) -> core::result::Result<(), String>{
        self.location_manager.choose_address(index)
    }
}


#[async_trait]
impl GetWeather for OpenWeatherProvider {
    type Weather = self::contracts::Weather;

    async fn get_weather(&self, date: Option<NaiveDate>) -> core::result::Result<Self::Weather, String>{
        let address = self.location_manager.get_chosen_address()?;

        match date {
            Some(d) => future_weather::get_forecast(address, d).await,
            None => current_weather::get_today_weather(address).await
        }
    }
}