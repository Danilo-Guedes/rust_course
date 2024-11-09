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

fn read_from_stdin() -> String {
    let mut input = String::new(); //this initialize an empty string

    println!("Enter the time of the day in seconds(0 to 86,399):");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    return input;
}

fn parse_string_as_u32(input: String) -> u32 {
    let total_seconds: u32 = input
        .trim()
        .parse() // this method try to convert the string to a number
        .expect("Input number only without any sign!");

    return total_seconds;
}

fn get_user_input() -> u32 {
    return parse_string_as_u32(read_from_stdin());
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

    mod parse_user_input {
        // keep all test cases to uniot test get_user_input

        // 1. when user input is 0, function must return 0
        #[test]
        fn test_user_enters_zero_function_must_return_zero() {
            assert_eq!(0, super::super::parse_string_as_u32("0\n\r".to_string()));
        }
        // 2. when user enters whole number, function must return whole number
        #[test]
        fn test_user_enters_whole_number_function_must_return_whole_number() {
            assert_eq!(123, super::super::parse_string_as_u32("123\n\r".to_string()));
        }
        // 3. when user enters a negative number, function must panic
        #[test]
        #[should_panic]
        fn when_user_enters_a_negative_number_function_must_panic() {
            let a_negative_number_as_string = "-123\n\r".to_string();
            super::super::parse_string_as_u32(a_negative_number_as_string);
        }
        // 4. when user enters a decimal number, function must panic
        #[test]
        #[should_panic]
        fn when_user_enters_a_decimal_number_function_must_panic() {
            let a_decimal_number_as_string = "123.45\n\r".to_string();
            super::super::parse_string_as_u32(a_decimal_number_as_string);
        }
        // 5. when user enters any character other than number, function must panic
        #[test]
        #[should_panic]
        fn when_user_enters_any_character_other_than_number_function_must_panic() {
            let a_character_as_string = "abc\n\r".to_string();
            super::super::parse_string_as_u32(a_character_as_string);
        }
        // 6. when user enters nothing(just hits enter key), function must panic
        #[test]
        #[should_panic]
        fn when_user_enters_nothing_function_must_panic() {
            let nothing_as_string = "\n\r".to_string();
            super::super::parse_string_as_u32(nothing_as_string);
        }
        // 7. when user enters a number which is greater than max value of u32, function must panic
        #[test]
        #[should_panic]
        fn when_user_enters_a_number_which_is_greater_than_max_value_of_u32_function_must_panic() {
            let a_big_number_as_string = "4294967296\n\r".to_string();
            super::super::parse_string_as_u32(a_big_number_as_string);
        }
    }
}
