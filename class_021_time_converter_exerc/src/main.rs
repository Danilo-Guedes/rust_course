/*
    Convert seconds to HH:MM:SS format
*/
use std::io;

fn main() {
    let total_seconds = get_user_input();
    println!("The time in 24hr format is {}", convert_seconds_to24hr_format(total_seconds))
}

fn convert_seconds_to24hr_format(total_seconds: u32) -> String {
    if total_seconds > 86399 {
        panic!("Your input should be between 0 to 86,399 ");
    }

    let hours = total_seconds / 3600;
    let remaining_seconds = total_seconds % 3600;
    let minutes = remaining_seconds / 60;
    let remaining_seconds = remaining_seconds % 60;

    let format_24h = format!("{:02}:{:02}:{:02}", hours, minutes, remaining_seconds);

    format_24h
}

fn get_user_input() -> u32 {
    let mut input = String::new();

    println!("Enter the time of the day in seconds(0 to 86,399):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let total_seconds: u32 = input.trim().parse().expect("Input number only without any sign!");

    return total_seconds;
}

#[cfg(test)]
mod test {
    mod time_converter {
        // keep all the test cases to unit test convert_seconds_to24hr_format

        // 1. when total_seconds is 0, function must return 00:00:00
        #[test]
        fn test_when_total_seconds_is_0() {
            assert_eq!(super::super::convert_seconds_to24hr_format(0), "00:00:00");
        }

        // 2. when total_seconds is 86400, function must panic
        #[test]
        #[should_panic(expected = "should be between 0 to 86,399")] // this will check if is a substring of the panic msg
        fn test_when_total_is_86400_must_panic() {
            super::super::convert_seconds_to24hr_format(86400);
        }

        // 3. when total_seconds is 86399, function must return 23:59:59

        // 4. when total_seconds is -ve, function must panic

        // THIS TEST NUMBER 4 IS NOT NECESSARY BECAUSE THE FUNCTION IS NOT EXPECTED TO HANDLE NEGATIVE NUMBERS AND IT WILL NOT EVEN COMPILE

        // #[test]
        // #[should_panic] // this will check if is a substring of the panic msg
        // fn test_when_total_is_negative_must_panic() {
        //     super::super::convert_seconds_to24hr_format(-2); << THIS WILL ERROR
        // }

    }

    //   mod user_input {
    //         use super::super::*;

    //         #[test]
    //         fn test_get_user_input() {
    //             assert_eq!(get_user_input(), 0);
    //         }
    //   }
}
