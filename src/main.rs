use std::{env, fmt};

fn main() {
    let original_temperature = match env::args().nth(1) {
        Some(argument) => argument.parse::<f64>().expect("The first argument must be a floating point number!"),
        None => panic!("Provide two arguments; a number, and either 'celsius' or 'fahrenheit'!"),
    };
    
    let (original_system, new_system) = match env::args().nth(2) {
        Some(argument) => match argument.to_ascii_lowercase().as_str() {
            "celsius" | "c" => (System::Celsius, System::Fahrenheit),
            "fahrenheit" | "f" => (System::Fahrenheit, System::Celsius),
            _ => panic!("The second argument must be either 'celsius' or 'fahrenheit'!"),
        },
        None => panic!("Provide two arguments; a number, and either 'celsius' or 'fahrenheit'!"),
    };
    
    let new_temperature = match original_system {
        System::Fahrenheit => from_fahrenheit(original_temperature),
        System::Celsius => from_celsius(original_temperature),
    };
    
    println!("{}{} equals {}{}", original_temperature, original_system, new_temperature, new_system);
}

enum System {
    Celsius,
    Fahrenheit,
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            System::Celsius => write!(f, "°C"),
            System::Fahrenheit => write!(f, "°F"),
        }
    }
}

fn from_fahrenheit(fahrenheit: f64) -> f64 {
    5.0/9.0 * (fahrenheit-32.0)
}

fn from_celsius(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

#[test]
fn correctly_converts_fahrenheit_to_celsius() {
    assert_eq!(from_fahrenheit(100.0), 37.0 + 7.0/9.0);
    assert_eq!(from_fahrenheit(-40.0), -40.0);
    assert_eq!(from_fahrenheit(212.0), 100.0);
}

#[test]
fn correctly_converts_celsius_to_fahrenheit() {
    assert_eq!(from_celsius(100.0), 212.0);
    assert_eq!(from_celsius(-40.0), -40.0);
    assert_eq!(from_celsius(37.0 + 7.0/9.0), 100.0);
}
