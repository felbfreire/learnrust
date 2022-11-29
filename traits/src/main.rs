pub trait Greet {
    fn greet(&self);
}

pub struct Person {
    name: String,
    _age: u32,
}

impl Greet for Person {
    fn greet(&self) {
        println!("{} says {}", self.name, String::from("Hello!"));
    }
}
fn main() {
    let jack = Person {
        name: String::from("Jack"),
        _age: 27,
    };

    jack.greet();

}
