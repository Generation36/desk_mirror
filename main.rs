use desk_mirror::location;
use desk_mirror::open_meteo;

fn main() {
    let denver= location::Location::new(39.7392, -104.9903).unwrap();
    // let philly= Location::new(39.9526, 75.1652);

    let raw_weather_data = open_meteo::get_weather_data(&denver);
    let weather_data: open_meteo::WeatherData = open_meteo::parse_weather_data(&raw_weather_data);

    println!("{:?}", weather_data);
}
