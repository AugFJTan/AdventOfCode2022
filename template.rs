// Sample text

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
	let file = File::open("input.txt")?;
	let reader = BufReader::new(file);
	
	let mut score: i32 = 0;

	for line in reader.lines() {
		let data = line.unwrap();
		
		// Your code here
	}
	
	println!("{}", score);  // Answer: 

	Ok(())
}
