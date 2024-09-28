use std::io;

fn do_addition(a: u32, b: u32) -> u32 {
    a + b
}

fn do_subtraction(a: u32, b: u32) -> u32 {
    a - b
}

fn do_multiplication(a: u32, b: u32) -> u32 {
    a * b
}

fn do_division(a: u32, b: u32) -> u32 {
    a / b
}

enum Operations {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

fn main() {
    println!("Please Enter your first number");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read input");
    let x: u32 = x.trim().parse().expect("Please enter a valid number");

    println!("Please Enter your second number");
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("Failed to read input");
    let y: u32 = y.trim().parse().expect("Please enter a valid number");

    println!("Choose an operation: 1) Addition 2) Subtraction 3) Multiplication 4) Division");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim(); // trim to remove new line

    match input {
        "1" => println!("Your addition is {}", do_addition(x, y)),
        "2" => println!("Your subtraction is {}", do_subtraction(x, y)),
        "3" => println!("Your multiplication is {}", do_multiplication(x, y)),
        "4" => println!("Your division is {}", do_division(x, y)),
        _ => println!("Invalid choice"),
    }
}
