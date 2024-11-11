fn main() {
    let a = 10;
    let b = 20;
    let c = a == b;
    let d = b > a;

    println!("a:{}, b:{}, c:{}, d:{}", a, b, c, d);

    if a == b {
        println!("a is equal to b");
    } else {
        println!("a is not equal to b");
    }

    // the negation operator    

    let trully = true;
    let negation = !trully;

    println!("trully:{}, negation:{}", trully, negation);
}
