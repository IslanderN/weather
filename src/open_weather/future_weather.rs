use serde::Deserialize;
use chrono::{NaiveDate, TimeZone, Utc, NaiveDateTime};
use reqwest::Response;
use futures::*;
use std::result::Result;
use crate::open_weather::configuration::API_KEY;
use crate::http_client::*;
use super::contracts::*;

#[derive(Deserialize, Debug)]
struct Temperature {
    min: f64,
    max: f64
}

#[derive(Deserialize, Debug)]
struct DailyForecast {
    #[serde(alias = "dt")]
    date_time_in_unix: i64,
    temperature: Temperature

}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
struct DailyForecasts {
    array: Vec<DailyForecast>

}

#[derive(Deserialize, Debug)]
pub struct OpenWeatherResponse {
    list: DailyForecasts
}

pub async fn get_forecast(address: &Address, day: NaiveDate) -> Result<Weather, String> {
    let request_url = 
        format!("https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&apikey={api_key}&units={units}&cnt={days}",
                lat = address.lat,
                lon = address.lon,
                units = "metric",
                days = 16,
                api_key = API_KEY);

    reqwest::get(&request_url)
        .map_err(stringify_reqwest_err)
        .and_then(|r| { pase_response(r, day) })
        .await

}

async fn pase_response(response: Response, day: NaiveDate) -> Result<Weather, String> {
    response.json::<OpenWeatherResponse>()
        .map_err(stringify_reqwest_err)
        .await
        .and_then(|r| { convert_to_forecast(r, day) })
}

fn convert_unix_to_date_time(time: i64) -> Result<NaiveDateTime, String>{
    Utc.timestamp_millis_opt(time).single()
        .ok_or("Couldn't parse date time".to_string())
        .map(|dt| {dt.naive_local()})
}

fn convert_to_forecast(response: OpenWeatherResponse, day: NaiveDate) -> Result<Weather, String> {
    response.list.array.into_iter()
        .find(|daily| {convert_unix_to_date_time(daily.date_time_in_unix).map_or(false, |d| {d.date() == day})})
        .ok_or(format!("Couldn't found a forecast for day {day}"))
        .and_then(convert_daily_forecast_to_forecast)
}

fn convert_daily_forecast_to_forecast(daily: DailyForecast) -> Result<Weather, String> {
    let date_time = convert_unix_to_date_time(daily.date_time_in_unix)?;
    Ok(Weather{date: date_time, temperature: super::contracts::Temperature::Range{min: daily.temperature.min, max: daily.temperature.max}})

}