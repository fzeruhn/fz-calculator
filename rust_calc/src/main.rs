use std::io;
mod calculator;

fn main() {

	let mut ins = String::new();
	println!("Enter an equation to calculate: ");
	io::stdin().read_line(&mut ins).expect("Failed to read line");

	let mut num1: i32 = 0;
	let mut num2: i32;
	let mut focus: i32;
	let mut num_string = String::new();
	let mut func: char = ' ';

	focus = 1;

	for c in ins.chars(){
		//If char is a digit
		if c.is_digit(10){
			num_string.push(c);
		} else {
			//If not digit and not whitespace, assume is function
			if !c.is_whitespace(){
				func = c;
			}

			if !num_string.is_empty() && focus == 1 {
				num1 = num_string.parse().unwrap();
				num_string.clear();
				focus = 2
			//If second number is complete, calculate and set num1 to result
			} else if !num_string.is_empty() && focus == 2 {
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
				}
			//If whitespace
			} else {
				continue;
			}
		}
	}
	println!("Result is: {}", num1);
}
