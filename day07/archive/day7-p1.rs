// Sample text

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

trait FSObject {
	fn get_name(&self) -> &String;
	fn get_size(&self) -> i32;
	fn get_type(&self) -> &str;
}

struct FSFile {
	name: String,
	size: i32,
}

impl FSFile {
	fn new(name: String, size: i32) -> Self {
		FSFile { name: name, size: size }
	}
}

struct FSDirectory {
	name: String,
	//contents: Vec<Box<dyn FSObject>>,
	parent: Option<Box<FSDirectory>>,
	files: Vec<FSFile>,
	directories: Vec<FSDirectory>,
}

impl FSDirectory {
	fn new(name: String) -> Self {
		FSDirectory { name: name, parent: None, files: Vec::new(), directories: Vec::new() }
	}
}

impl FSObject for FSFile {
	fn get_name(&self) -> &String {
		&self.name
	}
	
	fn get_size(&self) -> i32 {
		self.size
	}
	
	fn get_type(&self) -> &str {
		"FSFile"
	}
}

impl FSObject for FSDirectory {
	fn get_name(&self) -> &String {
		&self.name
	}
	
	fn get_size(&self) -> i32 {
		let mut total = 0;
		
		for file in self.files.iter() {
			total += file.get_size();
		}
		
		for directory in self.directories.iter() {
			total += directory.get_size();
		}
		
		total
	}
	
	fn get_type(&self) -> &str {
		"FSDirectory"
	}
}

fn main() -> io::Result<()> {
	let file = File::open("input.txt")?;
	let reader = BufReader::new(file);
	
	let mut score: i32 = 0;

	/*let mut root = FSDirectory::new("/".to_string());
	let mut current_dir = &mut root;
	//let mut all_directories = 

	for line in reader.lines() {
		let data = line.unwrap();
		
		if &data[0..1] == "$" {
			if &data[2..4] == "cd" {
				println!("CD command");
				// Next directory
				if &data[5..] != ".." {
					let dir_name = &data[5..];
					
					let mut new_dir = FSDirectory::new(dir_name.to_string());
					
					new_dir.parent = Some(Box::new(*current_dir));
					current_dir.directories.push(new_dir);
					
					*current_dir = new_dir;
				// Previous directory
				} else {
					println!("Go up one directory");
					*current_dir = *current_dir.parent.unwrap();
				}
				
				println!("======= Currently in {} ========", current_dir.get_name());
			} else {
				println!("LS command");
			}
		} else {
			if &data[0..3] == "dir" {
				println!("Directory");
			} else {
				println!("File");
				let mut file_data = data.split_whitespace();
				let size: i32 = file_data.next().unwrap().parse().unwrap();
				let name: String = file_data.next().unwrap().to_string();
				println!("{} {}", size, name);
				current_dir.files.push(FSFile::new(name, size));
			}
		}
	}*/
	
	let mut fsdir = FSDirectory::new("d".to_string());
	fsdir.files.push(FSFile::new("1".to_string(), 10));
	fsdir.files.push(FSFile::new("2".to_string(), 20));
	fsdir.files.push(FSFile::new("3".to_string(), 30));
	
	let mut subdir = FSDirectory::new("a".to_string());
	subdir.files.push(FSFile::new("4".to_string(), 40));
	
	fsdir.directories.push(subdir);
	
	println!("{}", fsdir.get_size());
	
	println!("{}", score);  // Answer: 

	Ok(())
}
