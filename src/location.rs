pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
}

impl Location {
  pub fn new(lat: f32, long: f32) -> Result<Self, &'static str> {
      if long < -90.0 && long > 90.0 {
          Err("Invalid longitude provided")
      } else if lat <= -180.0 && lat >= 180.0 {
          Err("Invalid latitude provided")
      } else {
          Ok(Location {
              latitude: lat,
              longitude: long,
          })
      }
  }

  pub fn display(&self) -> String {
      let lat_dir = match self.latitude {
          -90.0..0.0 => "°S",
          0.0..=90.0 => "°N",
          _ => "",
      };
      let long_dir = match self.longitude {
          -180.0..0.0 => "°E",
          0.0..180.0 => "°W",
          _ => "",
      };

      format!(
          "{}{}, {}{}",
          self.latitude, lat_dir, self.longitude, long_dir
      )
  }
}