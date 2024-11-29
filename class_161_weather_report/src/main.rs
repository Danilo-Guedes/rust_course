use::class_161_weather_report::get_temperature;

fn main() {
    println!("\n\n------------------\n\n");

    let temp = get_temperature();

    println!("Current temperature is : {}ºC", temp);
}

