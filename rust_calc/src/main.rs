use std::io;
mod calculator;

fn main() {

	let mut ins = String::new();
	println!("Enter an equation to calculate: ");
	io::stdin().read_line(&mut ins).expect("Failed to read line");

	let mut num1: i32 = 0;
	let mut num2: i32 = 0;
	let mut focus: i32;
	let mut num_string = String::new();
	let mut func: char = ' ';

	focus = 1;

	for c in ins.chars(){
		//If char is a digit
		if c.is_digit(10){
			num_string.push(c);
		//If is a space
		} else if c.is_whitespace() {
			if num_string.is_empty(){
				continue;
			}
			if focus == 1{ 
				num1 = num_string.parse().unwrap();
				num_string.clear();
				focus = 2;
			} else {
				num2 = num_string.parse().unwrap();
				num_string.clear();
				focus = 1;
				num1 = match func{
					'+' => calculator::addition(num1, num2),
					'-' => calculator::subtraction(num1, num2),
					'*' => calculator::multiplication(num1, num2),
					'/' => calculator::division(num1, num2),
					_ => {
						println!("Invalid action!");
						return;
					}
				};
			}
		//Assume char is a function
		} else {
			func = c;
		}	
	}
	println!("Result is: {}", num1);
}
