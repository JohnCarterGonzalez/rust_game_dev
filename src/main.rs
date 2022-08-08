use std::io::stdin;
#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction, 
    age: i8,
}   

//enumerator to add a under21 age prohibition to enter the treehouse
#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String},
    Refuse, 
    Probation,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action, 
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the Treehouse, {}", self.name), 
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the Treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol");
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
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
        visitor_list.push(Visitor::new("Bert", VisitorAction::Accept, 45));
        visitor_list.push(Visitor::new("Steve", VisitorAction::AcceptWithNote{note: String::from("Lactose-free milk is in the fridge")}, 15));
        visitor_list.push(Visitor::new("Tim", VisitorAction::Refuse, 30 ));
        
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
