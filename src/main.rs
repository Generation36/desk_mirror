use chrono::Local;
use desk_mirror::{location::Location, open_meteo, config};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    /*
        Process
        1. init loop -> get config, shouldn't need to update any changes just yet
        2. get current time
        3. get current weather 
            TODO: determine frequency of fetching weather updates
        4. render changes
    */
 
    let config = config::load_config("config.toml").unwrap();
    let now = Local::now().naive_local();
    // Consider adding a display error message if the config fails
    let location = Location::new(config.location.latitude, 
        config.location.longitude,
        config.location.timezone).unwrap();

    let weather_data = open_meteo::get_weather_data(&location, &config.weather_api);

    println!("{:?}", weather_data);

    // render display function call
    Ok(())
}
