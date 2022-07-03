use crate::Location;
use std::f64::consts::PI;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))];
pub struct Distance(f64);

impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} meters", self.meters())
    }
}

impl Distance {
    pub fn from_meters<P: Into<f64>>(m: P) -> Self {
        Distance(m.into())
    }
}