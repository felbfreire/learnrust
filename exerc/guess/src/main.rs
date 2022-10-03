// use standard io library(crates)
use std::io;
use std::cmp::Ordering;

// Crate downloaded by Cargo
use rand::Rng;


fn main() {
	println!("Guess a {n}! :D", n="number");

	let secret_number = rand::thread_rng().gen_range(1..=100);

	loop {

		println!("Pleasse, input your guess.");

		// mutable variable, which will be 
		// bound to a new, empty instance of string
		let mut guess = String::new();

		// calling stdin 
		io::stdin()

			// calling std::io::stdin methods

			// put stdin into mutable <guess> variable
			.read_line(&mut guess)
			.expect("Failed to read guess");

		// ** shadowing: using the same namespace **
		// guess u32 equals treated guess string (but number type)
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue, // "_" = catchall
		};

		println!("You guessed {guess}");

		match guess.cmp(&secret_number) {

			Ordering::Less => println!("Too small"),
			Ordering::Greater => println!("Too big"),
			Ordering::Equal => {

				println!("You win!");
				break;

			}	
		}
	}
}
