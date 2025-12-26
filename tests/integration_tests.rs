use temperature_converter::convert_temperature;
use assert_approx_eq::assert_approx_eq;

#[test]
fn test_celsius_to_kelvin() {
    // 30°C = 303.15K
    let result = convert_temperature(30.0, "C", "K").unwrap();
    assert_approx_eq!(result, 303.15, 1e-10);
}

#[test]
fn test_fahrenheit_to_kelvin() {
    // 100°F = 310.927777...K
    let result = convert_temperature(100.0, "F", "K").unwrap();
    assert_approx_eq!(result, 310.9277777777778, 1e-10);
}

#[test]
fn test_celsius_to_fahrenheit() {
    // -10°C = 14°F
    let result = convert_temperature(-10.0, "C", "F").unwrap();
    assert_approx_eq!(result, 14.0, 1e-10);
}

#[test]
fn test_same_unit() {
    // 25°C → 25°C
    let result = convert_temperature(25.0, "C", "C").unwrap();
    assert_approx_eq!(result, 25.0, 1e-10);
}

#[test]
fn test_invalid_from_unit() {
    // Неправильная исходная единица
    let result = convert_temperature(25.0, "X", "C");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Неподдерживаемая единица"));
}

#[test]
fn test_invalid_to_unit() {
    // Неправильная целевая единица
    let result = convert_temperature(25.0, "C", "X");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Неподдерживаемая единица"));
}

#[test]
fn test_kelvin_to_celsius() {
    // 0K = -273.15°C
    let result = convert_temperature(0.0, "K", "C").unwrap();
    assert_approx_eq!(result, -273.15, 1e-10);
}

#[test]
fn test_extreme_temperatures() {
    // Абсолютный ноль в разных системах
    let result = convert_temperature(-273.15, "C", "K").unwrap();
    assert_approx_eq!(result, 0.0, 1e-10);
    
    let result = convert_temperature(-459.67, "F", "K").unwrap();
    assert_approx_eq!(result, 0.0, 1e-2); // Немного меньшая точность из-за double
}