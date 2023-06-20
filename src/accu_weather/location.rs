use crate::accu_weather::configuration::API_KEY;
use serde::Deserialize;
use reqwest::Response;
use futures::*;
use std::fmt::{Display, Formatter, Result};
use crate::http_client::*;
use super::contracts::*;


#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Area {
    #[serde(alias ="LocalizedName")]
    name: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct City {
    key: String,
    #[serde(alias ="LocalizedName")]
    name: String,
    country: Area,
    #[serde(alias ="AdministrativeArea")]
    region: Area
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{city}, {region}, {country}",
                city = &self.name,
                region = &self.region.name,
                country = &self.country.name)
    }
    
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
struct LocationResponse {
    cities: Vec<City>
}

pub async fn get_location(input: String) -> std::result::Result<Vec<Address>, String> {
    let request_url = 
        format!("http://dataservice.accuweather.com/locations/v1/cities/autocomplete?apikey={api_key}&q={address}",
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
    Address { key: city.key, full_address: address }
}