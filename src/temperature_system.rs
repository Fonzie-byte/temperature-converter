use std::fmt;

pub enum TemperatureSystem {
    Celsius,
    Fahrenheit,
}

impl fmt::Display for TemperatureSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TemperatureSystem::Celsius => write!(f, "°C"),
            TemperatureSystem::Fahrenheit => write!(f, "°F"),
        }
    }
}
