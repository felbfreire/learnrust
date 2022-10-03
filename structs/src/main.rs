/*structs are a custom datatype that packages meaningful values*/

struct Person {
    id: u8,
    name: String,
    email: String,
    likes_tomatoes: bool,
}


fn main() {

    let mut john = Person { 
                        id: 86,
                        name: String::from("John"),
                        email: String::from("Johnathan@somemail.com"),
                        likes_tomatoes: true,
    };

    println!("{} email is {} ..", john.name, john.email);

    //changing jonhs email. our struct is a mutable type!
    john.email = String::from("Jhonatz@somemail.com");

    println!(".. now {} email is {} ..", john.name, john.email);
    println!(".. {} likes tomatoes? {}! his id is {} by the way ..", john.name, john.likes_tomatoes, john.id);
 
}

