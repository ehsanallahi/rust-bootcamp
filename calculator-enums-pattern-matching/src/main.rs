use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => x / y,
    }
}

fn main() {
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let first_number: f64 = input.trim().parse().expect("Please enter a valid number.");

    println!("Enter the operation (+, -, *, /):");
    input.clear(); // Clearing the previous input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operation = input.trim(); // Trimmed operation input

    println!("Enter the second number:");
    let mut input_second = String::new(); // Separate mutable input for second number
    io::stdin().read_line(&mut input_second).expect("Failed to read line");
    let second_number: f64 = input_second.trim().parse().expect("Please enter a valid number.");

    let operation_enum = match operation {
        "+" => Operation::Add(first_number, second_number), // if select Add
        "-" => Operation::Subtract(first_number, second_number), // if select Subtract
        "*" => Operation::Multiply(first_number, second_number), // if Multiply
        "/" => Operation::Divide(first_number, second_number), // if select Divide
        _ => panic!("Invalid operation!"),
    };

    let result = calculate(operation_enum);

    println!("Result: {}", result);
}
