use crate::location::Location;
use std::time::Duration;
use reqwest::{blocking::Client, StatusCode};
use serde::{Serialize, Deserialize};

pub fn get_weather_data(loc: &Location) -> String {
  println!("Fetching weather data for {}", loc.display());
  let weather_api_url = "https://api.open-meteo.com/v1/forecast";
  let params = [
    ("latitude", loc.latitude.to_string()),
    ("longitude", loc.longitude.to_string()), 
    ("daily", "temperature_2m_min,temperature_2m_max".to_string()),
    ("current", "temperature_2m,weather_code".to_string()),
    ("timezone", "America/Denver".to_string()),
    ("forecast_days", "1".to_string()),
    ("temperature_unit", "fahrenheit".to_string())
  ];
  
  let response = Client::new()
    .get(weather_api_url)
    .query(&params)
    .timeout(Duration::from_secs(5))
    .send()
    .expect("Unable to fetch weather data");

  // if response.status() != StatusCode::is_success(response) {

  // }

  println!("Status: {}", &response.status());
  // println!("Headers:\n{:#?}", &response.headers());

  let data = &response.text().unwrap();
  // println!("Body:\n{:?}", data);
    
  data.clone()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherData {
  // Temperature units will always be in fahrenheit 
  current_temp: f64, 
  daily_max_temp: f64,
  daily_min_temp: f64,
  weather_code: u64,
}

pub fn parse_weather_data(weather_data: &String) -> WeatherData {

  let data: serde_json::Value = serde_json::from_str(&weather_data).unwrap();
  println!("Weather Data: {:?}", data);
  let current_temperature = data["current"]["temperature_2m"].as_f64().unwrap();
  let weather_code = data["current"]["weather_code"].as_u64().unwrap();
  let daily_temps = data["daily"].to_owned();
  let daily_max_temp = daily_temps["temperature_2m_max"][0].as_f64().unwrap();
  let daily_min_temp = daily_temps["temperature_2m_min"][0].as_f64().unwrap();
  
  WeatherData { 
    current_temp: current_temperature, 
    daily_max_temp: daily_max_temp, 
    daily_min_temp: daily_min_temp, 
    weather_code: weather_code 
  }
}
