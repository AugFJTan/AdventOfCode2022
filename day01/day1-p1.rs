// Count highest total Calories

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
	
	let mut highest: i32 = 0;
	let mut total: i32 = 0;

    for line in reader.lines() {
		let data = line.unwrap();
		
		if data != "" {
			let value: i32 = data.parse().unwrap();
			total += value;
		} else {
			if total > highest {
				highest = total;
			}
			total = 0;
		}
    }
	
	println!("{}", highest);  // Answer: 71506

    Ok(())
}
