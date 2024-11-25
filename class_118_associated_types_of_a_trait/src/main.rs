trait Animal {
    fn make_sound(&self);
    fn set_age(&mut self, _new_age: u8) {
        println!("This feature is not supported by this animal");
    }
    fn get_age(&self) -> u8 {
        println!("This feature is not supported by this animal");
        0
    }
    type Weight;
    fn get_weight(&self) -> Self::Weight;
    fn set_weight(&mut self, new_weight: Self::Weight);
}

#[derive(Default)]
struct Dog {
    age: u8,
    weight: f32,
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Dog barks");
    }

    fn set_age(&mut self, new_age: u8) {
        self.age = new_age;
    }

    fn get_age(&self) -> u8 {
        self.age
    }
    type Weight = f32;
    fn get_weight(&self) -> Self::Weight {
        self.weight
    }
    fn set_weight(&mut self, new_weight: Self::Weight) {
        self.weight = new_weight;
    }
}

#[derive(Default)]
struct Cat {
    age: u8,
    weight: u8
}

impl Animal for Cat {
    // for the cat it will use the default implementations of the trait
    fn make_sound(&self) {
        println!("Cat meows");
    }
    type Weight = u8;
    fn get_weight(&self) -> Self::Weight {
        self.weight
    }
    fn set_weight(&mut self, new_weight: Self::Weight) {
        self.weight = new_weight;
    }
}

fn produce_sound<T>(animal: &dyn Animal<Weight= T>) {
    // this dyn means that the type of this instance is not known at compile time
    animal.make_sound();
}

fn main() {
    println!("\n\n-------------------\n\n");

    let mut my_dog = Dog::default();
    my_dog.make_sound();
    my_dog.set_age(5);
    println!("My dog is {} years old", my_dog.get_age());
    println!("the weight before change: {}", my_dog.get_weight());
    my_dog.set_weight(15.0);
    println!("the weight after change: {:.2}", my_dog.get_weight());

    println!("\n\n-------------------\n\n");

    let mut my_cat = Cat::default();
    my_cat.make_sound();
    my_cat.set_age(3);
    println!("My cat is {} years old", my_cat.get_age());
    println!("the weight before change: {}", my_cat.get_weight());
    my_cat.set_weight(5);
    println!("the weight after change: {}", my_cat.get_weight());

    println!("\n\n-------------------\n\n");
}
