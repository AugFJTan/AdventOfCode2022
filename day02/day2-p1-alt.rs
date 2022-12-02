// Total score of strategy
// Opponent/Player : A/X=Rock, B/Y=Paper, C/Z=Scissors
// Shape selected  : Rock=1, Paper=2, Scissors=3
// Outcome of round: Lose=0, Draw=3, Win=6

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
	let file = File::open("input.txt")?;
	let reader = BufReader::new(file);
	
	let mut score: i32 = 0;
	
	let mut opponent = HashMap::new();  // Alternatively:
	opponent.insert("Rock", "A");       // let opponent_rock = "A";
	opponent.insert("Paper", "B");      // let opponent_paper = "B";
	opponent.insert("Scissors", "C");   // let opponent_scissors = "C";
	
	let mut player = HashMap::new();
	player.insert("Rock", "X");
	player.insert("Paper", "Y");
	player.insert("Scissors", "Z");

	for line in reader.lines() {
		let data = line.unwrap();
		let mut round = data.split_whitespace();
		
		let opponent_move = round.next().unwrap();
		let player_move   = round.next().unwrap();
		
		// Shape selected
		if player_move == player["Rock"] {
			score += 1;
		} else if player_move == player["Paper"] {
			score += 2;
		} else { // player_move == player["Scissors"]
			score += 3;
		}
		
		// Outcome of Round
		// Win
		if (player_move == player["Rock"]     && opponent_move == opponent["Scissors"]) ||
		   (player_move == player["Paper"]    && opponent_move == opponent["Rock"]    ) ||
		   (player_move == player["Scissors"] && opponent_move == opponent["Paper"]   ) {
			score += 6;
		// Draw
		} else if 
		   (player_move == player["Rock"]     && opponent_move == opponent["Rock"]    ) ||
		   (player_move == player["Paper"]    && opponent_move == opponent["Paper"]   ) ||
		   (player_move == player["Scissors"] && opponent_move == opponent["Scissors"]) {
			score += 3;
		}
		// Note: Don't add anything for lose condition!
	}
	
	println!("{}", score);  // Answer: 11873

	Ok(())
}
