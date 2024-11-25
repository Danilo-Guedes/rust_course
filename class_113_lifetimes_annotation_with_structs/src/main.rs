struct MyStruct<'a, 'b> {
    data1: &'a str,
    data2: &'b str,
}

fn main() {
    let data1 = String::from("Hello");
    let data2 = String::from("World");

    let my_struct = MyStruct {
        data1: &data1,
        data2: &data2,
    };

    let ret = fun(&my_struct);

    println!("ret = {}", ret);
}

fn fun<'a,'b,'c>(data: &'a MyStruct<'b, 'c>) -> &'b str {
    data.data1
}
