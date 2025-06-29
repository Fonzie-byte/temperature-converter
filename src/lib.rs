use std::fmt;

/// Converts the given degrees Fahrenheit to Celsius and returns it as a float.
///
/// # Examples
/// ```
/// use temperature_converter::from_fahrenheit;
///
/// assert_eq!(from_fahrenheit(97.5), 36.38888888888889);
/// assert_eq!(from_fahrenheit(32.0), 0.0);
/// assert_eq!(from_fahrenheit(-40.0), -40.0);
/// ```
pub fn from_fahrenheit(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

/// Converts the given degrees Celsius to Fahrenheit and returns it as a float.
///
/// # Examples
/// ```
/// use temperature_converter::from_celsius;
/// 
/// assert_eq!(from_celsius(100.0), 212.0);
/// assert_eq!(from_celsius(0.0), 32.0);
/// assert_eq!(from_celsius(-40.0), -40.0);
/// ```
pub fn from_celsius(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

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
