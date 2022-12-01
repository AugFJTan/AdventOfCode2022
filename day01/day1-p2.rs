// Count top 3 highest total Calories

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
	
	let mut total: i32 = 0;
	let mut calories = Vec::new();

    for line in reader.lines() {
		let data = line.unwrap();
		
		if data != "" {
			let value: i32 = data.parse().unwrap();
			total += value;
		} else {
			calories.push(total);
			total = 0;
		}
    }
	
	calories.sort();
	calories.reverse();
	
	println!("{}", calories[0] + calories[1] + calories[2]);  // Answer: 209603

    Ok(())
}
