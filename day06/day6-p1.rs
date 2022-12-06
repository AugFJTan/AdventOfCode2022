// Number of characters before start-of-packet marker
// Detect different characters in a window of 4

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
	let file = File::open("input.txt")?;
	let reader = BufReader::new(file);
	
	let mut marker: i32 = 4;
	let data = reader.lines().next().unwrap();
	let binding = data?;
	let signal = binding.as_bytes();
	
	for i in 0..signal.len()-3 {
		let mut duplicate = false;
		
		for x in 0..=3 {
			for y in 0..=3 {
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
	
	println!("{}", marker);  // Answer: 1723

	Ok(())
}
