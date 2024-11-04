fn main() {
    let path1 = "C:\\Program Files\\Microsoft"; // backslash is an escape character

    let double_quote = "the value is \"Hello\""; // we can use escape character to use double quotes

    println!("{} and {}", path1, double_quote);

    // we can also use raw string notation

    let path2 = r"C:\Program Files\Microsoft"; // r is used to denote raw string

    println!("{}", path2);

    // but the problem is that r notation you can not use quotes inside due to escape character not being used

    // double_quote2 = r"the value is "Hello""; // this will give error

    // println!("{}", double_quote2);

    // but we can use multiple quotes

    let double_quote2 = r#"the value is "Hello" "World""#; // this will work

    println!("{}", double_quote2);

    // we can also use multi-line string

    let giant_string = r###"
            This is a large, multi-line string.
            It includes quotes: "Hello" "World".
            And maybe even something like "##" in the text.
            "###;

    println!("{}", giant_string);
}
