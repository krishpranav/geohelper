// #![deny(missing_docs)]
mod formula;

pub use formula::Distance;

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        match formula::vincenty_inverse(self, to, 0.00001, 0.0) {
            Ok(res) => Ok(res.distance),
            Err(e) => Err(e),
        }
    }

    pub fn haversine_distance_to(&self, to: &Location) -> Distance {
        formula::haversine_distance_to(self, to)
    }

    pub fn is_in_circle(&self, center: &Location, radius: Distance) -> Result<bool, String> {
        match formula::vincenty_inverse(self, center, 0.00001, 0.0) {
            Ok(res) => Ok(res.distance.meters() < radius.meters()),
            Err(e) => Err(e),
        }
    }

    pub fn center(coords: &[&Location]) -> Location {
        formula::center_of_coords(coords)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_distance_haversine() {
        let l1 = Location::new(27.740068, 85.337576);
        let l2 = Location::new(27.740286, 85.337059);

        let distance = l1.haversine_distance_to(&l2);
        assert_eq!(distance.meters(), 56.36);
    }

}