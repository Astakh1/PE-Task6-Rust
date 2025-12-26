#[derive(Debug, PartialEq, Eq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl TemperatureUnit {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_uppercase().as_str() {
            "C" | "CELSIUS" => Ok(TemperatureUnit::Celsius),
            "F" | "FAHRENHEIT" => Ok(TemperatureUnit::Fahrenheit),
            "K" | "KELVIN" => Ok(TemperatureUnit::Kelvin),
            _ => Err(format!("Неподдерживаемая единица: {}", s)),
        }
    }

    pub fn to_celsius(&self, value: f64) -> f64 {
        match self {
            TemperatureUnit::Celsius => value,
            TemperatureUnit::Fahrenheit => (value - 32.0) * 5.0 / 9.0,
            TemperatureUnit::Kelvin => value - 273.15,
        }
    }

    pub fn from_celsius(&self, value: f64) -> f64 {
        match self {
            TemperatureUnit::Celsius => value,
            TemperatureUnit::Fahrenheit => (value * 9.0 / 5.0) + 32.0,
            TemperatureUnit::Kelvin => value + 273.15,
        }
    }
}

pub fn convert_temperature(value: f64, from: &str, to: &str) -> Result<f64, String> {
    let from_unit = TemperatureUnit::from_str(from)?;
    let to_unit = TemperatureUnit::from_str(to)?;

    if from_unit == to_unit {
        return Ok(value);
    }

    let celsius = from_unit.to_celsius(value);
    let result = to_unit.from_celsius(celsius);
    
    Ok(result)
}