use serde::Deserialize;
use chrono::{NaiveDate, DateTime, Utc};
use reqwest::Response;
use futures::*;
use std::result::Result;
use crate::accu_weather::configuration::API_KEY;
use crate::http_client::*;
use super::contracts::*;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct TemperatureValue {
    value: f64,
    unit_type: i32
}

#[derive(Deserialize, Debug)]
struct Temperature {
    #[serde(alias = "Minimum")]
    min_value: TemperatureValue,
    #[serde(alias = "Maximum")]
    max_value: TemperatureValue
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct DailyForecast {
    date: DateTime<Utc>,
    temperature: Temperature

}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
struct DailyForecasts {
    array: Vec<DailyForecast>

}

#[derive(Deserialize, Debug)]
pub struct AccueWeatherResponse {
    #[serde(alias = "DailyForecasts")]
    daily_forecasts: DailyForecasts
}

pub async fn get_forecast(address: &Address, day: NaiveDate) -> Result<Weather, String> {
    let request_url = 
        format!("http://dataservice.accuweather.com/forecasts/v1/daily/5day/{location_id}?apikey={api_key}&language={language}",
                location_id = address.key,
                api_key = API_KEY,
                language = "en-gb");

    reqwest::get(&request_url)
        .map_err(stringify_reqwest_err)
        .and_then(|r| { pase_response(r, day) })
        .await

}

async fn pase_response(response: Response, day: NaiveDate) -> Result<Weather, String> {
    response.json::<AccueWeatherResponse>()
        .map_err(stringify_reqwest_err)
        .await
        .and_then(|r| { convert_to_forecast(r, day) })
}

fn convert_to_forecast(response: AccueWeatherResponse, day: NaiveDate) -> Result<Weather, String> {
    response.daily_forecasts.array.into_iter()
        .find(|daily| {daily.date.date_naive() == day})
        .ok_or(format!("Couldn't found a forecast for day {day}"))
        .and_then(convert_daily_forecast_to_forecast)
}

fn convert_daily_forecast_to_forecast(daily: DailyForecast) -> Result<Weather, String> {
    let min_temp = match_temperature(&daily.temperature.min_value)?;
    let max_temp = match_temperature(&daily.temperature.max_value)?;
    Ok(Weather{date: daily.date.naive_local(), temperature: super::contracts::Temperature::Range{min: min_temp, max: max_temp}})

}

fn match_temperature(val: &TemperatureValue) -> Result<f64, String> {
    match val.unit_type {
        17 => Ok(val.value),
        18 => Ok(convert_fahrenheit_to_celsius(val.value)),
        _ => Err("Incorrect unit_type for temperature. Required 17 or 18".to_string())
    }
}

fn convert_fahrenheit_to_celsius(val: f64) -> f64 {
    (val - 32.0) * 5.0 / 9.0
}