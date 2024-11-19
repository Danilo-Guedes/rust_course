enum LightState {
    On { brightness: u8 },
    Off,
}




fn main() {
    let bulb = LightState::On { brightness: 180 };

    match bulb {
        LightState::On { brightness: 180 } => {
            println!("The bulb is on with brightness: 180");
        }

        LightState::On { .. } => {
            println!("The bulb is NOT on with brightness: 180");
        }
        LightState::Off => {
            println!("The bulb is off");
        }
    }
}


