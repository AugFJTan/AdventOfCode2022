// Sum of priorities of item types in groups of 3

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

fn main() -> io::Result<()> {
	let file = File::open("input.txt")?;
	let reader = BufReader::new(file);
	
	let mut score: i32 = 0;
	let mut group = Vec::new();

	for line in reader.lines() {
		let data = line.unwrap();
		
		group.push(data);
		
		if group.len() == 3 {
			let a: Vec<char> = group[0].chars().collect::<Vec<_>>();
			let b: Vec<char> = group[1].chars().collect::<Vec<_>>();
			let c: Vec<char> = group[2].chars().collect::<Vec<_>>();
			
			let a_set: HashSet<_> = a.iter().cloned().collect();
			let b_set: HashSet<_> = b.iter().cloned().collect();
			let c_set: HashSet<_> = c.iter().cloned().collect();
			
			let ab_intersect: Vec<_> = a_set.intersection(&b_set).collect();
			let ac_intersect: Vec<_> = a_set.intersection(&c_set).collect();
			
			let ab_set: HashSet<_> = ab_intersect.iter().cloned().collect();
			let ac_set: HashSet<_> = ac_intersect.iter().cloned().collect();
			
			let abc_intersect: Vec<_> = ab_set.intersection(&ac_set).collect();
			let shared: i32 = (**abc_intersect[0]) as i32;
			
			// lowercase
			if shared >= 97 && shared <= 122 {
				score += shared - 96;
			// uppercase
			} else {
				score += shared - 38;
			}
			
			group.clear();
		}
	}
	
	println!("{}", score);  // Answer: 2342

	Ok(())
}

