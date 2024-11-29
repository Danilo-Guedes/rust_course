use::class_161_weather_report::get_temperature;

fn main() {
    let temp = get_temperature();

    if temp > 30.0 {
        println!("Alert: High temperature warning!");
    } else if temp < 10.0 {
        println!("Alert: Low temperature warning!");
    } else {
        println!("Temperature is normal");
    }

}