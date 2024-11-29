use::class_161_weather_report::get_temperature;

fn main() {
    println!("\n\n------------------\n\n");

    let temp = get_temperature();
    println!("Weather Report:");
    println!("Current temperature is : {}]C", temp);
    println!("Condition is Sunny");
}
