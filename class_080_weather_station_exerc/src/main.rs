#![allow(dead_code)]

use std::io::Write;

/// Build a simple weather station application
/// The goal is to record and manage weather data for different cities.
/// Hints: Watch the demo video in the section named 'Vectors'

#[derive(Debug)]
enum WeatherCondition {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
}

impl WeatherCondition {
    fn show(&self) -> &str {
        match self {
            WeatherCondition::Sunny => "Sunny ☀️",
            WeatherCondition::Cloudy => "Cloudy ☁️",
            WeatherCondition::Rainy => "Rainy 🌧️",
            WeatherCondition::Snowy => "Snowy ❄️",
        }
    }
}

struct Weather {
    temperature: f64,
    humidity: u32,
    condition: WeatherCondition,
}

impl Weather {
    fn new(temperature: f64, humidity: u32, condition: WeatherCondition) -> Self {
        Weather {
            temperature,
            humidity,
            condition,
        }
    }
}

struct CityWeather {
    city: String,
    weather: Weather,
}

impl CityWeather {
    fn new(city: String, weather: Weather) -> Self {
        CityWeather { city, weather }
    }
}

struct WeatherStation {
    cities: Vec<CityWeather>,
}

impl WeatherStation {
    fn new() -> Self {
        WeatherStation { cities: Vec::new() }
    }

    fn add_city(&mut self, city_weather: CityWeather) {
        self.cities.push(city_weather);
    }
}

fn display_prompt() {
    let font_green = "\x1b[32m";
    let font_reset = "\x1b[0m";

    print!(
        r#"{}
\\\\\\Simple Weather Station\\\\\\\\\\
\\\\\\Display all weather reports -- 0
\\\\\\Add a new weather report    -- 1
\\\\\\Display city weather report -- 2
\\\\\\Update weather report       -- 3
\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
Enter your choice: {}"#,
        font_green, font_reset
    );

    stdout_flush();
}

fn display_all_weather_reports(weather_station: &WeatherStation) {
    if weather_station.cities.len() == 0 {
        println!("Nothing to display")
    } else {
        for item in &weather_station.cities {
            println!(
                "City: {city} \t\t\t\tTemperature: {temp}\t\t\t\tHumidity: {humi}\t\t\t\tCondition: {cond}",
                city = item.city,
                temp = format!("{}°", item.weather.temperature),
                humi = format!("{}%", item.weather.humidity),
                cond = item.weather.condition.show()
            )
        }
    }
}

fn get_new_weather_report_from_user() -> CityWeather {
    todo!();
}

fn add_new_weather_report(weather_station: &mut WeatherStation) {
    prompt_and_flush("Enter city name:");
    let city_name = stdin_read_string();

    prompt_and_flush("Enter temperature(°C):");
    let city_temp = stdin_read_float();

    prompt_and_flush("Enter humidity(%):");
    let city_hum = stdin_read_integer();

    prompt_and_flush("Describe weather Condition(Snowy, Cloudy, Rainy, Sunny):");
    let city_cond = stdin_read_string();

    let found_condition: Option<WeatherCondition> = match city_cond.to_lowercase().as_str() {
        input if input.contains("sun") => Some(WeatherCondition::Sunny),
        input if input.contains("cloud") => Some(WeatherCondition::Cloudy),
        input if input.contains("rain") => Some(WeatherCondition::Rainy),
        input if input.contains("snow") => Some(WeatherCondition::Snowy),
        _ => None,
    };

    if let Some(condition) = found_condition {
        println!("Found condition is {:?}", condition);

        let weather: Weather = Weather::new(city_temp, city_hum, condition);

        let city_weather = CityWeather::new(city_name, weather);

        weather_station.add_city(city_weather);
    } else {
        eprintln!("Invalid weather condition entered");
    }
}

fn display_city_weather_report(weather_station: &WeatherStation) {
    todo!();
}

fn update_city_weather_report(weather_station: &mut WeatherStation) {
    todo!();
}

fn main() {
    let mut weather_station: WeatherStation = WeatherStation::new();

    loop {
        display_prompt();

        match read_user_input() {
            0 => display_all_weather_reports(&weather_station),
            1 => add_new_weather_report(&mut weather_station),
            2 => display_city_weather_report(&weather_station),
            3 => update_city_weather_report(&mut weather_station),
            _ => {
                println!("Exiting...");
                break;
            }
        }

        print!("Press Enter key to continue...");
        stdout_flush();
        let _ = stdin_read_string();
    }
}

fn stdout_flush() {
    let _ = std::io::stdout().flush();
}

fn stdin_read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading from stdin");
    input.trim().to_string()
}

fn stdin_read_float() -> f64 {
    let input = stdin_read_string();
    input
        .parse::<f64>()
        .expect("Error while converting string to f32")
}

fn stdin_read_integer() -> u32 {
    let input = stdin_read_string();
    input
        .parse::<u32>()
        .expect("Error while converting string to u32")
}

fn read_user_input() -> u32 {
    stdin_read_integer()
}

fn prompt_and_flush(text: &str) {
    print!("{}  ", text);
    stdout_flush();
}
