// Number of assignment pairs that fully contain another

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
	let file = File::open("input.txt")?;
	let reader = BufReader::new(file);
	
	let mut full_overlaps: i32 = 0;

	for line in reader.lines() {
		let data = line.unwrap();
		let mut pair = data.split(",");
		
		let first_pair = pair.next().unwrap();
		let second_pair = pair.next().unwrap();
		
		let mut first_range = first_pair.split("-");
		let mut second_range = second_pair.split("-");
		
		let first_pair_start: i32 = first_range.next().unwrap().parse().unwrap();
		let first_pair_end: i32 = first_range.next().unwrap().parse().unwrap();
		
		let second_pair_start: i32 = second_range.next().unwrap().parse().unwrap();
		let second_pair_end: i32 = second_range.next().unwrap().parse().unwrap();
		
		if (first_pair_start <= second_pair_start && first_pair_end >= second_pair_end) ||
		   (second_pair_start <= first_pair_start && second_pair_end >= first_pair_end) {
			full_overlaps += 1;
		}
	}
	
	println!("{}", full_overlaps);  // Answer: 542

	Ok(())
}
