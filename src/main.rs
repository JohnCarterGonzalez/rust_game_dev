use std::io::stdin;
#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String
}   

impl Visitor {
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
    loop {
        println!("Hello, whats your name? (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();

        let mut visitor_list = Vec::new();
        visitor_list.push(Visitor::new("Bert", "Hello Bert, enjoy your treehouse"));
        visitor_list.push(Visitor::new("Steve", "Hello Steve, enjoy your treehouse"));
        visitor_list.push(Visitor::new("Tim", "Hello Tim, enjoy your treehouse"));
        
        let mut _allow_them_in = false;

        // checks to see of the option has data and makes the contents of the option available to the code in the clause visitor 
        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor_list.", name);
                    visitor_list.push(Visitor::new(&name, "New Friend"));
                    println!("The final list of Visitors");
                    println!("{:#?}", visitor_list);
                }
            }
         }
    }
}
