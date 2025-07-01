use std::{collections::HashMap, time::Duration, error::Error};
use reqwest::{blocking::Client};
use serde::{Serialize, Deserialize};
use crate::config::ApiConfig;
use crate::location::Location;

pub fn build_request_params(loc: &Location, config: &ApiConfig) -> HashMap<&'static str, String> {
  let mut params = HashMap::new();
  params.insert("latitude", loc.latitude.to_string());
  params.insert("longitude", loc.longitude.to_string());
  params.insert("timezone", loc.timezone.to_string());
  params.insert("daily", config.daily_values.clone());
  params.insert("current", config.current_values.clone());
  params.insert("forecast_days", config.forecast_days.to_string());
  params.insert("temperature_unit", config.units.clone());

  params
}

pub fn get_weather_data(loc: &Location, config: &ApiConfig) -> Result<WeatherData, Box< dyn Error>> {
  println!("Fetching weather data for {}", loc.display());
  let params = build_request_params(loc, config);
  
  let response = Client::new()
    .get(&config.url)
    .query(&params)
    .timeout(Duration::from_secs(5))
    .send()
    .expect("Unable to fetch weather data");

  let data = response.text()?;
  parse_weather_data(&data)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherData {
  // Temperature units will always be in fahrenheit 
  current_temp: f64, 
  daily_max_temp: f64,
  daily_min_temp: f64,
  weather_code: u64,
}

pub fn parse_weather_data(weather_data: &String) -> Result<WeatherData, Box< dyn Error>> {

  let data: serde_json::Value = serde_json::from_str(&weather_data).unwrap();
  println!("Weather Data: {:?}", data);
  let current_temperature = data["current"]["temperature_2m"].as_f64().unwrap();
  let weather_code = data["current"]["weather_code"].as_u64().unwrap();
  let daily_temps = data["daily"].to_owned();
  let daily_max_temp = daily_temps["temperature_2m_max"][0].as_f64().unwrap();
  let daily_min_temp = daily_temps["temperature_2m_min"][0].as_f64().unwrap();
  
  Ok(WeatherData { 
    current_temp: current_temperature, 
    daily_max_temp: daily_max_temp, 
    daily_min_temp: daily_min_temp, 
    weather_code: weather_code 
  })
}
