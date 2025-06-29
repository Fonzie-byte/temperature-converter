use std::env;
use temperature_converter::{TemperatureSystem, from_celsius, from_fahrenheit};

fn main() {
    let original_temperature = match env::args().nth(1) {
        Some(argument) => argument
            .parse::<f64>()
            .expect("The first argument must be a floating point number!"),
        None => panic!("Provide two arguments; a number, and either 'celsius' or 'fahrenheit'!"),
    };

    let (original_system, new_system) = match env::args().nth(2) {
        Some(argument) => match argument.to_ascii_lowercase().as_str() {
            "celsius" | "c" => (TemperatureSystem::Celsius, TemperatureSystem::Fahrenheit),
            "fahrenheit" | "f" => (TemperatureSystem::Fahrenheit, TemperatureSystem::Celsius),
            _ => panic!("The second argument must be either 'celsius' or 'fahrenheit'!"),
        },
        None => panic!("Provide two arguments; a number, and either 'celsius' or 'fahrenheit'!"),
    };

    let new_temperature = match original_system {
        TemperatureSystem::Fahrenheit => from_fahrenheit(original_temperature),
        TemperatureSystem::Celsius => from_celsius(original_temperature),
    };

    println!(
        "{}{} equals {}{}",
        original_temperature, original_system, new_temperature, new_system
    );
}
