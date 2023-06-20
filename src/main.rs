#![allow(dead_code)]

mod accu_weather;
mod open_weather;
mod contracts;
mod http_client;
mod location_manager;

use chrono::NaiveDate;
use contracts::*;
use futures::future::*;
use crate::accu_weather::AccueWeatherProvider;
use crate::open_weather::OpenWeatherProvider;
//use crate::contracts::GetWeather;

fn read(stdin: &std::io::Stdin) -> Result<String, String> {
    let mut input = String::new();
    stdin.read_line(&mut input)
        .map_err(|err| {format!("Coudn't read input: {error}", error = err.to_string())})
        .map(|_| {input.trim().to_string()})

}

fn parse_usize(input: String) -> Result<usize, String> {
    input.parse::<i32>()
        .map(|i| {i as usize})
        .map_err(|e| {format!("Incorrect int format: {e}")})
        
}

fn parse_date(input: String) -> Result<NaiveDate, String> {
    input.parse::<NaiveDate>()
        .map_err(|e| {format!("Incorrect date format: {e}")})
        
}

fn decrease_chosen_index(i: usize) -> Result<usize, String> {
    if i == 0 
    { 
        Err( "Incorrect chosen variant".to_string())
    }
    else {
        Ok(i)
    }
}

fn get_weather_output(f: &impl Forecast) -> String {
    format!("Day: {day}\nTemperature: {temp}",
            day = f.get_date(),
            temp = f.get_temperature())
}


#[tokio::main]
async fn main() {
    let stdin = std::io::stdin();

    println!("Enter address: ");

    let address_input = read(&stdin);

    //let mut accu_weather_provider = AccueWeatherProvider::new();
    let mut accu_weather_provider = OpenWeatherProvider::new();

    //let response = accu_weather::forecast::get_forecast().await;
    let response = 
        async move { address_input}
        .and_then(|a| {accu_weather_provider.find_addresses(a)}).await;

    let v = response.unwrap();


    let length = v.len();

    for i in 0..length {
        println!("{}: {}", (i + 1), v[i].get_full_address())
    }
    println!("Choose one: ");
    
    read(&stdin)
    .and_then(parse_usize)
    .and_then(decrease_chosen_index)
    .and_then(|i|{accu_weather_provider.choose_address(i)})
    .unwrap();



    println!("Enter date: ");

    let chosen_date =
        read(&stdin)
        .and_then(parse_date);

    let r = 
        async move { chosen_date}
        //.and_then(accu_weather::today::get_today_weather)
        .and_then(|day| {accu_weather_provider.get_weather(Some(day))})
        .await;



    let forecast = r.unwrap();

   // let s = response.and_then(|response: Response| ->  {});

    println!("{}", get_weather_output(&forecast));
    //Ok(())
}