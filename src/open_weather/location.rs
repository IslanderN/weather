use crate::open_weather::configuration::API_KEY;
use serde::Deserialize;
use reqwest::Response;
use futures::*;
use std::fmt::{Display, Formatter, Result};
use crate::http_client::*;
use super::contracts::Address;

#[derive(Deserialize, Debug)]
struct City {
    lat: f64,
    lon: f64,
    name: String,
    country: String,
    state: Option<String>
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self.state {
            Some(state) =>
                write!(f, "{city}, {state}, {country}",
                        city = &self.name,
                        state = state,
                        country = &self.country),
            None =>
                write!(f, "{city}, {country}",
                        city = &self.name,
                        country = &self.country),

        }
    }
    
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct LocationResponse {
    cities: Vec<City>
}

pub async fn get_location(input: String) -> std::result::Result<Vec<Address>, String> {
    let request_url = 
        format!("http://api.openweathermap.org/geo/1.0/direct?appid={api_key}&q={address}",
                address = input,
                api_key = API_KEY);

    get_request(&request_url)
        .and_then(pase_response)
        .await

}

async fn pase_response(response: Response) -> std::result::Result<Vec<Address>, String> {
    response.json::<LocationResponse>()
        .map_err(stringify_reqwest_err)
        .await
        .map(convert_to_addresses)
}

fn convert_to_addresses(response: LocationResponse) -> Vec<Address> {
    response.cities.into_iter().map(convert_city_to_address).collect()
}

fn convert_city_to_address(city: City) -> Address {
    let address = city.to_string();
    Address { lat: city.lat, lon: city.lon, full_address: address }
}