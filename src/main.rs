fn main() {
    println!("Hello, world!");
}

#[test]
fn correctly_converts_fahrenheit_to_celsius() {
    assert_eq!(from_fahrenheit(100.0), 37.0 + 7.0/9.0);
}

#[test]
fn correctly_converts_celsius_to_fahrenheit() {
    assert_eq!(from_celsius(100.0), 212.0);
}
