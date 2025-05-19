use std::io::stdin;
fn main() {    
    let mut message = String::new();
    println!("Please enter a message:");
    stdin().read_line(&mut message).unwrap();
    println!("You entered: {}", message);
}
