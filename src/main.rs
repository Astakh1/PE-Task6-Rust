use clap::{Parser, ValueEnum};
use temperature_converter::{convert_temperature, TemperatureUnit};

#[derive(Parser)]
#[command(author, version, about = "Конвертер температур", long_about = None)]
struct Cli {
    /// Значение температуры
    value: f64,

    /// Исходная единица измерения
    #[arg(value_enum, short, long)]
    from: UnitArg,

    /// Целевая единица измерения
    #[arg(value_enum, short, long)]
    to: UnitArg,

    /// Показать подробности
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum UnitArg {
    C,
    F,
    K,
}

impl From<UnitArg> for &'static str {
    fn from(unit: UnitArg) -> Self {
        match unit {
            UnitArg::C => "C",
            UnitArg::F => "F",
            UnitArg::K => "K",
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match convert_temperature(cli.value, cli.from.into(), cli.to.into()) {
        Ok(result) => {
            if cli.verbose {
                println!("Конвертация температуры:");
                println!("  Исходное значение: {} {:?}", cli.value, cli.from);
                println!("  Результат: {} {:?}", result, cli.to);
                println!("  Формула: {}°{} → {}°{}", 
                    cli.value, cli.from, result, cli.to);
            } else {
                println!("{}", result);
            }
        }
        Err(e) => {
            eprintln!("Ошибка: {}", e);
            std::process::exit(1);
        }
    }
}