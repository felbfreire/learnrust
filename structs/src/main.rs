/*structs are a custom datatype that packages meaningful values*/
struct Person {
    id: u8,
    name: String,
    email: String,
    likes_tomatoes: bool,
}

fn build_person(id: u8, name: String, email: String, likes_tomatoes: bool) -> Person {
    Person {
        id: id,
        name: name,
        email: email,
        likes_tomatoes: likes_tomatoes,
    }
}

// Method Syntax
impl Person {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

fn main() {

    let mut john = Person { 

        id: 86,
        name: String::from("John"),
        email: String::from("Johnathan@somemail.com"),
        likes_tomatoes: true,
    };

    // _variable = ill not use this!
    let _johnson = build_person(

        87,
        String::from("Johnson"),
        String::from("Johnson@email.com"),
        true
    );

    println!("{} email is {} ..", john.name, john.email);

    //changing jonhs email. our struct is a mutable type!
    john.email = String::from("Jhonatz@somemail.com");

    println!(".. now {} email is {} ..", john.name, john.email);
    println!(".. {} likes tomatoes? {}! his id is {} by the way ..", john.name, john.likes_tomatoes, john.id);
    println!(" John name by method syntax: {}", john.get_name());
 
}

