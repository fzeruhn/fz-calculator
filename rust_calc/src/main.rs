use std::io;
mod calculator;

fn main() {

	let mut input = String::new();
	println!("Enter an action (+|-|*|/): ");
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line");
	let c: char = input.trim().chars().next().unwrap();

	let mut input2 = String::new();
	println!("Enter first number to be used in equation: ");
	io::stdin()
		.read_line(&mut input2)
		.expect("Failed to read line");
	let num1: i32 = input2.trim().parse().expect("Enter a valid int");

	let mut input3 = String::new();
	println!("Enter second number to be used in equation: ");
		io::stdin()
		.read_line(&mut input3)
		.expect("Failed to read line");
	let num2: i32 = input3.trim().parse().expect("Enter a valid int");

	println!("Function to calculate is: {} {} {}", num1, c, num2);

	let result = match c{
		'+' => calculator::addition(num1, num2),
		'-' => calculator::subtraction(num1, num2),
		'*' => calculator::multiplication(num1, num2),
		'/' => calculator::division(num1, num2),
		_ => {
			println!("Invalid action!");
			return;
		}
	};
	println!("Result is: {}", result);
}
