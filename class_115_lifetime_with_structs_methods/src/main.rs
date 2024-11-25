
#[derive(Debug)]
struct MyStruct<'a, 'b> {
    data1: &'a str,
    data2: &'b str,
}

impl<'a, 'b> MyStruct<'a, 'b> {
    fn get_data1(&self) -> &'a str {
        self.data1
    }

    fn set_data(&mut self, s1: &'a str, s2: &'b str) {
        self.data1 = s1;
        self.data2 = s2;
    }

    fn get_longest<'c>(&'c self, s: &'c str) -> &str {
        let longest_self = if self.data1.len() > self.data2.len() {
            self.data1
        } else {
            self.data2
        };

        if longest_self.len() > s.len() {
            longest_self
        } else {
            s
        }
    }
}


fn main() {
    let mut struct_ins = MyStruct {
        data1: "Hi",
        data2: "World",
    };

    let ret = struct_ins.get_data1();
    println!("{}", ret);

    let s1 = "Good";
    let s2 = "Morning";

    struct_ins.set_data(s1, s2);
    println!("{:?}", struct_ins);

    println!("{}", struct_ins.get_longest("abcdefgh"));
}

