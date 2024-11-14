/*
Write a program to check the validity of password input by users.
Following are the criteria for checking the password:
1. At least 1 letter between [a-z]
2. At least 1 number between [0-9]
3. At least 1 letter between [A-Z]
4. At least 1 character from [$#@]
5. Minimum length of transaction password: 6
6. Maximum length of transaction password: 12
*/

//Expected output

/*
   ###### Set New Password ######
   1. At least 1 letter between [a-z]
   2. At least 1 number between [0-9]
   3. At least 1 letter between [A-Z]
   4. At least 1 character from [$#@]
   5. Minimum length of transaction password: 6
   6. Maximum length of transaction password: 12

   Enter New Password: <User enters the password>
   Re-enter New Password: <User re-enters the same password>
   Password is Weak/Strong/Do not match
   Password change successful/unsuccessful
*/

//tasks to be completed
//1. take user input
//2. check the passwd strength..COMPLETED
//3. match the passwords ...COMPLETED
//4. output the status

fn main() {}

fn is_passwords_match(pass1: &str, pass2: &str) -> bool {
    return pass1 == pass2;
}

fn is_password_contains_lowercase_letters(passw: &str) -> bool {
    for ch in passw.chars() {
        if ch.is_ascii_lowercase() {
            return true;
        }
    }
    return false;
}
fn is_password_contains_uppercase_letters(passw: &str) -> bool {
    for ch in passw.chars() {
        if ch.is_ascii_uppercase() {
            return true;
        }
    }
    return false;
}

fn is_password_contains_digits(passw: &str) -> bool {
    for ch in passw.chars() {
        if ch.is_ascii_digit() {
            return true;
        }
    }
    return false;
}

fn is_password_contains_allowed_chars(passw: &str) -> bool {
    let allowed_chars = ['$', '#', '@'];
    for ch in passw.chars() {
        if allowed_chars.contains(&ch) {
            return true;
        }
    }
    return false;
}

fn is_password_contains_not_allowed_chars(passw: &str) -> bool {
    let allowed_chars = vec!['$', '#', '@'];
    for ch in passw.chars() {
        if ch.is_ascii_alphanumeric() || allowed_chars.contains(&ch) {
            return true;
        }
    }
    return false;
}

fn print_status(weak_or_string: u8, match_or_not: bool) -> &'static str {
    if weak_or_string == 0 {
        return "Password is Weak \n Password change unsuccessful";
    } else {
        if match_or_not {
            return "Password is Strong \n Password change successful";
        } else {
            return "Password is Strong \n Password change unsuccessful";
        }
    }
}

fn check_password_strength(passw: &str) -> u8 {
    //weak = 0, strong= 1

    if is_password_contains_lowercase_letters(passw)
        && is_password_contains_uppercase_letters(passw)
        && is_password_contains_digits(passw)
        && is_password_contains_allowed_chars(passw)
        && !is_password_contains_not_allowed_chars(passw)
    {
        return 1;
    } else {
        return 0;
    }
}

#[cfg(test)]
mod test {
    use crate::*; // this import will bring all the functions to the test scope of the current file
    #[test]
    fn test_when_both_passwords_are_same_returns_true() {
        assert_eq!(true, is_passwords_match("abc", "abc"));
    }
    #[test]
    fn test_when_both_passwords_are_not_same_returns_false() {
        assert_eq!(false, is_passwords_match("zxc", "abc"));
    }

    mod passwd_strength {
        use crate::*;
        #[test]
        fn test_when_password_is_empty_return_weak() {
            assert_eq!(0, check_password_strength(""));
        }

        #[test]
        fn test_when_password_contains_only_a_to_z_letters_returns_weak() {
            assert_eq!(0, check_password_strength("abc"));
        }

        #[test]
        fn test_when_password_contains_only_a_to_a_uppercase_letters_returns_weak() {
            assert_eq!(0, check_password_strength("ABC"));
        }

        #[test]
        fn test_when_password_contains_only_digits_returns_weak() {
            assert_eq!(0, check_password_strength("123"));
        }

        #[test]
        fn test_when_password_is_greater_than_returns_weak() {
            assert_eq!(0, check_password_strength("abcdEF@123456789"));
        }

        #[test]
        fn test_when_password_contains_lower_case_letters_returns_true() {
            assert_eq!(true, is_password_contains_lowercase_letters("abc"));
        }

        #[test]
        fn test_when_password_does_not_contains_lower_case_letters_returns_false() {
            assert_eq!(false, is_password_contains_lowercase_letters("ABC123"));
        }

        #[test]
        fn test_when_password_contains_any_uppercase_letters_returns_true() {
            assert_eq!(true, is_password_contains_uppercase_letters("ABC123"));
        }

        #[test]
        fn test_when_password_contains_no_uppercase_letters_returns_false() {
            assert_eq!(false, is_password_contains_uppercase_letters("abc123$$"));
        }

        #[test]
        fn test_when_password_contains_any_digit_returns_true() {
            assert_eq!(true, is_password_contains_digits("ABC123"));
        }

        #[test]
        fn test_when_password_contains_no_digit_returns_false() {
            assert_eq!(false, is_password_contains_digits("ABCabc#@"));
        }

        #[test]
        fn test_when_password_contains_allowed_special_chars_returns_true() {
            assert_eq!(true, is_password_contains_allowed_chars("ABCabc123$"));
        }

        #[test]
        fn test_when_password_contains_not_allowed_special_chars_returns_true() {
            assert_eq!(
                true,
                is_password_contains_not_allowed_chars("ABCabc123++!!")
            );
        }

        #[test]
        fn test_when_password_match_and_is_strong_print_strong_and_successful() {
            assert_eq!(
                "Password is Strong \n Password change successful",
                print_status(1, true),
            )
        }

        #[test]
        fn test_when_password_match_and_is_weak_print_weak_and_unsuccessful() {
            assert_eq!(
                "Password is Weak \n Password change unsuccessful",
                print_status(0, true),
            )
        }
    }
}
