// Crates on top of each stack
// Use stack data structure with pop, push; but elements retain order

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn insert_crates(stacks: &mut Vec<Vec<char>>, data: String) {
	for i in 0..9 {
		let letter = data.as_bytes()[1+(i*4)] as char;
		
		if letter != ' ' {
			stacks[i].push(letter);
		}
	}
}

fn move_crates(stacks: &mut Vec<Vec<char>>, data: String) {
	let instruction: Vec<String> = data.split_whitespace().map(str::to_string).collect();
	let count: i32 = instruction[1].parse().unwrap();
	let from: i32 = instruction[3].parse::<i32>().unwrap() - 1;
	let to: i32 = instruction[5].parse::<i32>().unwrap() - 1;

	let mut crates_to_move: Vec<char> = Vec::new();
	
	for _ in 0..count {
		crates_to_move.push(stacks[from as usize].pop().unwrap() as char);
	}
	
	crates_to_move.reverse();
	
	stacks[to as usize].append(&mut crates_to_move);
}

fn main() -> io::Result<()> {
	let file = File::open("input.txt")?;
	let reader = BufReader::new(file);
	
	let mut line_count = 0;
	
	let mut stacks: Vec<Vec<char>> = Vec::new();
	
	for _ in 0..9 {
		stacks.push(Vec::new());
	}

	for line in reader.lines() {
		let data = line.unwrap();
		
		line_count += 1;
		
		match line_count {
			1..=8 => insert_crates(&mut stacks, data),
			9..=10 => continue,
			_ => move_crates(&mut stacks, data)
		}
		
		if line_count == 8 {
			for i in 0..9 {
				stacks[i].reverse();
			}
		}
	}
	
	for stack in stacks {
		print!("{}", stack[stack.len()-1]);  // Answer: PRTTGRFPB
	}
	println!();

	Ok(())
}
