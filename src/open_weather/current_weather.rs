use serde::Deserialize;
use chrono::{Utc, TimeZone};
use reqwest::Response;
use futures::*;
use std::result::Result;
use crate::open_weather::configuration::API_KEY;
use crate::http_client::*;
use super::contracts::*;


#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct MainWeather {
    #[serde(alias = "temp")]
    temperature: f64
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct TodayResponse {
    main: MainWeather,
    #[serde(alias = "dt")]
    date_time_in_unix: i64
}


pub async fn get_today_weather(address: &Address) -> Result<Weather, String> {
    let request_url = 
        format!("https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&apikey={api_key}",
                lat = address.lat,
                lon = address.lon,
                api_key = API_KEY);

    get_request(&request_url)
        .and_then(pase_response)
        .await

}

async fn pase_response(response: Response) -> Result<Weather, String> {
    response.json::<TodayResponse>()
        .map_err(stringify_reqwest_err)
        .await
        .and_then(convert_to_forecast)
}

fn convert_to_forecast(response: TodayResponse) -> Result<Weather, String> {
    let date_time = 
        Utc.timestamp_millis_opt(response.date_time_in_unix).single()
            .ok_or("Couldn't parse date time".to_string())?;
    Ok(Weather{date: date_time.naive_local(), temperature: Temperature::Value(response.main.temperature)})
}