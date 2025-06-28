fn main() {
    println!("Hello, world!");
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
