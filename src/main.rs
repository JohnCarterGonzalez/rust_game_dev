use std::io::stdin;
#[derive(Debug)]
struct Vistor {
    name: String,
    greeting: String
}   

impl Vistor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name () -> String {

    let mut your_name = String ::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name
        .trim()
        .to_lowercase() 

}
fn main() {
    println!("Hello, whats your name?");
    let name = what_is_your_name();

    let visitor_list = vec! [
        Vistor::new("bert", "Hello Bert, enjoy"),
        Vistor::new("steve", "Hello Steve, enjoy"),
        Vistor::new("michael", "Hello Micheal, enjoy"),
    ];

    let mut _allow_them_in = false;

    let known_visitor = visitor_list
        .iter()
        .find(|visitor| visitor.name == name);
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You are nor on the visitor list, leave.")
    }
}
