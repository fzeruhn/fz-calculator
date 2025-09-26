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
	println!("Enter two numbers to use in equation, space seperated: ");
	io::stdin()
		.read_line(&mut input2)
		.expect("Failed to read line");
	//Parse the two numbers into sperate ints
	let num1, num2;

	match c{
		'+' => calculator::addition(),
		'-' => calculator::subtraction(),
		'*' => calculator::multiplication(),
		'/' => calculator::division(),
		_ => println!("Invalid action!"),
	}
}
