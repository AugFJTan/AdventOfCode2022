// Total score of strategy
// Opponent/Player : A/X=Rock, B/Y=Paper, C/Z=Scissors
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
		let player   = round.next().unwrap();
		
		// Shape selected
		if player == "X" {
			score += 1;
		} else if player == "Y" {
			score += 2;
		} else { // player == "Z"
			score += 3;
		}
		
		// Outcome of Round
		// Win
		if (player == "X" && opponent == "C") ||  // Rock vs Scissors
		   (player == "Y" && opponent == "A") ||  // Paper vs Rock
		   (player == "Z" && opponent == "B") {   // Scissors vs Paper
			score += 6;
		// Draw
		} else if 
		   (player == "X" && opponent == "A") ||  // Rock
		   (player == "Y" && opponent == "B") ||  // Paper
		   (player == "Z" && opponent == "C") {   // Scissors
			score += 3;
		}
		// Note: Don't add anything for lose condition!
	}
	
	println!("{}", score);  // Answer: 11873

	Ok(())
}
