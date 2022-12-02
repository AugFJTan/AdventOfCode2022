// Total score of strategy
// Opponent        : A=Rock, B=Paper, C=Scissors
// Result          : X=Lose, Y=Draw, Z=Win
// Shape selected  : Rock=1, Paper=2, Scissors=3
// Outcome of round: Lose=0, Draw=3, Win=6

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
	let file = File::open("input.txt")?;
	let reader = BufReader::new(file);
	
	let mut score: i32 = 0;

	for line in reader.lines() {
		let data = line.unwrap();
		let mut round = data.split_whitespace();
		
		let opponent = round.next().unwrap();
		let result   = round.next().unwrap();
		
		// Lose
		if result == "X" {
			if opponent == "A" {         // Rock vs Scissors
				score += 3;
			} else if opponent == "B" {  // Paper vs Rock
				score += 1;
			} else { // opponent == "C"  // Scissors vs Paper
				score += 2;
			}
		}
		// Draw
		else if result == "Y" {
			score += 3;
			if opponent == "A" {         // Rock
				score += 1;
			} else if opponent == "B" {  // Paper
				score += 2;
			} else { // opponent == "C"  // Scissors
				score += 3;
			}
		}
		// Win
		else { // result == "Z"
			score += 6;
			if opponent == "A" {         // Rock vs Paper
				score += 2;
			} else if opponent == "B" {  // Paper vs Scissors
				score += 3;
			} else { // opponent == "C"  // Scissors vs Rock
				score += 1;
			}
		}
	}
	
	println!("{}", score);  // Answer: 12014

	Ok(())
}
