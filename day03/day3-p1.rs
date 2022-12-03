// Sum of priorities of item types

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

fn main() -> io::Result<()> {
	let file = File::open("input.txt")?;
	let reader = BufReader::new(file);
	
	let mut score: i32 = 0;

	for line in reader.lines() {
		let data = line.unwrap();
		let (left, right) = data.split_at(data.chars().count()/2);
		
		let left_letters: Vec<char> = left.chars().collect::<Vec<_>>();
		let right_letters: Vec<char> = right.chars().collect::<Vec<_>>();
		
		let left_set: HashSet<_> = left_letters.iter().cloned().collect();
		let right_set: HashSet<_> = right_letters.iter().cloned().collect();
		
		let intersect: Vec<_> = left_set.intersection(&right_set).collect();
		let shared: i32 = (*intersect[0]) as i32;
		
		// lowercase
		if shared >= 97 && shared <= 122 {
			score += shared - 96;
		// uppercase
		} else {
			score += shared - 38;
		}
	}
	
	println!("{}", score);  // Answer: 8153

	Ok(())
}

