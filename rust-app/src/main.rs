use std::io::stdin;

struct Person {
    name: String,
    age: u32,
}

fn main() {    
    // let mut message = String::new();
    // println!("Please enter a message:");
    // stdin().read_line(&mut message).unwrap();
    // println!("You entered: {}", message);

    let parsed_num: i32 = "42".parse().unwrap();
    println!("Parsed number: {}", parsed_num);

    let parsed_float: f64 = "3.14".parse().unwrap();
    println!("Parsed float: {}", parsed_float);

    let parsed_bool: bool = "true".parse().unwrap();
    println!("Parsed boolean: {}", parsed_bool);

    let parsed_string: String = "Hello, world!".to_string();
    println!("Parsed string: {}", parsed_string);

    let parsed_char: char = 'A';
    println!("Parsed char: {}", parsed_char);

    let parsed_tuple: (i32, f64) = (42, 3.14);
    println!("Parsed tuple: ({}, {})", parsed_tuple.0, parsed_tuple.1);

    let parsed_array: [i32; 3] = [1, 2, 3];
    println!("Parsed array: [{}, {}, {}]", parsed_array[0], parsed_array[1], parsed_array[2]);
    println!("Array Length: {}", parsed_array.len());
    println!("Array size: {}", std::mem::size_of_val(&parsed_array));

    let person: Person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Person Name: {}", person.name);
    println!("Person Age: {}", person.age);

}
