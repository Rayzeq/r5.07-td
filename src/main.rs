use rust_ci_example::{Calculator, CalculatorError};
use std::io;

fn main() -> Result<(), CalculatorError> {
    println!("Simple Calculator");
    println!("Enter calculation (e.g., 5 + 3):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() != 3 {
        println!("Please enter in the format: number operator number");
        return Ok(());
    }

    let a: f64 = parts[0].parse().expect("Invalid number");
    let operator = parts[1];
    let b: f64 = parts[2].parse().expect("Invalid number");

    match Calculator::calculate(a, b, operator) {
        Ok(result) => println!("Result: {result}"),
        Err(CalculatorError::DivisionByZero) => println!("Error: Division by zero"),
        Err(CalculatorError::InvalidOperator) => println!("Error: Invalid operator"),
    }

    Ok(())
}
