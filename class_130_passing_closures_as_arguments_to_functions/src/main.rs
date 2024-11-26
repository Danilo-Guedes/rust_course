fn main() {
    println!("\n\n----------------------\n\n");

    // WE HAVE TWO OPTIONS TO PASS CLOSURES AS ARGUMENTS TO FUNCTIONS
    // 1. Using function pointers (if the closure doesn't capture any variables)
    // 2. Using generics and trait bounds

    // 1. Using function pointers

    let add_one = |x| x + 1;

    let result = apply(add_one, 2);

    println!("Result: {}", result);

    println!("\n\n----------------------\n\n");

    // 2. Using generics and trait bounds

    let y = 2;

    let double_the_value = |x| x * y; // fn trait

    let result_2 = apply_w_fn_trait(double_the_value, 10);

    println!("with Fn trait: {}", result_2);

    println!("\n\n----------------------\n\n");

    let mut z = 5;

    let change_the_value = |x| {
        z += x;
        z
    }; // fnMut trait

    let result_3 = apply_w_fnmut_trait(change_the_value, 30);

    println!("with fnMut trait: {}", result_3);

    println!("\n\n----------------------\n\n");

    let mut s = String::from("Hello ");

    let apend_abc = move || {
        s += "ABC";
        println!("with fnOnce trait: {}", s);
        s
    }; // fnOnce trait

   apply_w_fnonce_trait(apend_abc);


    println!("\n\n----------------------\n\n");
}

fn apply(f: fn(i32) -> i32, a: i32) -> i32 {
    f(a)
}

fn apply_w_fn_trait<F>(f: F, a: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(a)
}

fn apply_w_fnmut_trait<F>(mut f: F, a: i32) -> i32
where
    F: FnMut(i32) -> i32,
{
    f(a)
}

fn apply_w_fnonce_trait<F>(f: F)
where
    F: FnOnce() -> String,
{
    f();
}
