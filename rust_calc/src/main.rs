use std::io;
use std::process;
mod calculator;

fn calculate(num1: i32, func: char, num2: i32) -> i32 {
	let result: i32 = match func{
		'+' => calculator::addition(num1, num2),
		'-' => calculator::subtraction(num1, num2),
		'*' => calculator::multiplication(num1, num2),
		'/' => {
			if num2 == 0 {
				println!("Cannot divide by zero");
				process::exit(1);
			}
			calculator::division(num1, num2)
		}
		_ => {
			println!("Invalid action!");
			process::exit(1);
		}
	};
	println!("Calculated {} {} {} = {}", num1, func, num2, result);
	return result;
}

fn main() {

	let mut ins = String::new();
	println!("Enter an equation to calculate: ");
	io::stdin().read_line(&mut ins).expect("Failed to read line");

	let mut num1: i32 = 0;
	let mut num2: i32 = 0;
	let mut focus: i32;
	let mut num_string = String::new();
	let mut func: char = ' ';
	let mut next_func: char = ' ';
	let mut pending: bool = false;

	focus = 1;

	for c in ins.chars(){
		//If char is a digit
		if c.is_digit(10){
			num_string.push(c);
		} else {
			//If not digit, not whitespace, assume is function
			if !c.is_whitespace() {
				//Queue calculation
				if pending {
					next_func = c;
				} else {
					func = c;
					pending = true;
				}
			}

			if !num_string.is_empty() && focus == 1 {
				num1 = num_string.parse().unwrap();
				num_string.clear();
				focus = 2
			//If second number is complete, calculate and set num1 to result
			} else if !num_string.is_empty() && focus == 2 {
				num2 = num_string.parse().unwrap();
				num_string.clear();
				pending = false;
				num1 = calculate(num1, func, num2);
				func = next_func;
			//If whitespace
			} else {
				continue;
			}
		}
	}
	if pending {
		num1 = calculate(num1, func, num2);
	}
	println!("Result is: {}", num1);
}
