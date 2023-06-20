use serde::Deserialize;
use chrono::{DateTime, Utc};
use reqwest::Response;
use futures::*;
use std::result::Result;
use crate::accu_weather::configuration::API_KEY;
use crate::http_client::*;

use super::contracts::*;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct MetricTemperature {
    value: f64
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Temperature {
    metric: MetricTemperature

}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct DailyForecast {
    #[serde(alias = "LocalObservationDateTime")]
    date: DateTime<Utc>,
    temperature: Temperature

}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
struct TodayResponse {
    array: Vec<DailyForecast>
}

pub async fn get_today_weather(address: &Address) -> Result<Weather, String> {
    let request_url = 
        format!("http://dataservice.accuweather.com/currentconditions/v1/{location_id}?apikey={api_key}",
                location_id = address.key,
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
    response.array.into_iter()
        .nth(0)
        .ok_or("Couldn't found a forecast for that day".to_string())
        .map(convert_daily_forecast_to_weather)
}

fn convert_daily_forecast_to_weather(daily: DailyForecast) -> Weather {
    Weather {date: daily.date.naive_local(), temperature: super::contracts::Temperature::Value(daily.temperature.metric.value)}

}