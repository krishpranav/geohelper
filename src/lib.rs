pub struct Location(f64, f64);

impl Location {
    pub fn new<T: Into<f64>>(lat: T, lon: T) -> Self {
        Location(lat.into(), lon.into())
    }

    pub fn latitude(&self) -> f64 {
        self.0
    }

    pub fn longitude(&self) -> f64 {
        self.1
    }

    pub fn distance_to(&self, to: &Location) -> Result<Distance, String> {
    }
}