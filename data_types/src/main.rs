use std::io;

fn main() {

    println!("I am The Calculator");
    println!("");

    loop {
        println!("Enter the first number:");
        let mut first_number = String::new();
        io::stdin().read_line(&mut first_number).unwrap();
        let first_number: i32 = first_number.trim().parse().unwrap();
    
        println!("Enter the second number:");
        let mut second_number = String::new();
        io::stdin().read_line(&mut second_number).unwrap();
        let second_number: i32 = second_number.trim().parse().unwrap();
    
        println!("Enter the operation:");
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).unwrap();
    
    
        let mut matched = true;
        let mut result: i32 = 0;
        match operation.trim() {
            "+" => result = (first_number + second_number).into(),
            "-" => result = (first_number - second_number).into(),
            "/" => result = (first_number ) / (second_number ),
            "*" => result = (first_number * second_number).into(),
            _ => matched = false,
        };
    
    
        if !matched {
            println!("Unsupported operation '{operation}'");
        } else {
            println!("Result is {result}");
        }
    }
}
