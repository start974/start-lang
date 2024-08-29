use super::super::location::{Located, Location};

use std::str;

pub struct Constant<T> {
    val: T,
    location: Option<Location>,
}

pub struct ParseError;

impl<T> Constant<T> {
    pub fn make(val: T) -> Self {
        Self {
            val,
            location: None,
        }
    }
}

impl<T> Located for Constant<T> {
    fn get_location(&self) -> &Option<Location> {
        &self.location
    }

    fn set_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }
}

impl<T> str::FromStr for Constant<T>
where
    T: str::FromStr,
{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.parse::<T>().map_err(|_| ParseError)?;
        Ok(Self::make(v))
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Constant<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}
