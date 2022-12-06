// Number of characters before start-of-message marker
// Detect different characters in a window of 14

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
	let file = File::open("input.txt")?;
	let reader = BufReader::new(file);
	
	let mut marker: i32 = 14;
	let data = reader.lines().next().unwrap();
	let binding = data?;
	let signal = binding.as_bytes();
	
	for i in 0..signal.len()-13 {
		let mut duplicate = false;
		
		for x in 0..=13 {
			for y in 0..=13 {
				if x == y {
					continue;
				}
				if signal[i+x] == signal[i+y] {
					duplicate = true;
				}
			}
		}
		
		if !duplicate {
			break;
		}
		
		marker += 1;
	}
	
	println!("{}", marker);  // Answer: 3708

	Ok(())
}
