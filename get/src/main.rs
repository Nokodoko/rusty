mod greeter;
use greeter::greet_user;

fn main() {
   let name: Result<String, Error> = greet_user();
    if let Ok(name) = Result{
        println!("Hello {name}"); 
    } else {
        println!("{:?}", Result); 
    }
}

