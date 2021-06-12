/* 
Item: Easy to use logger.
Maintainer: github.com/python128
Target Users: CLI users
Target UI: Linux, any CLI UI
*/

use std::io::Write;
use std::io::Read;
use std::fs::OpenOptions;
use chrono::Utc;
use colored::*;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let action = &args[1];
	let item = &args[2];
	let log_file = "log";

	if action == "create" {
		create_file(item.to_string());
	} else if action == "view" {
		print_file(item.to_string());
	} else if action == "log" {
		append_to_file(log_file.to_string(), item.to_string());
	}
}

fn create_file(file_name: String) {
	std::fs::File::create(&file_name).expect(&"\nCreation failed".red());
	println!("\n\nCreated file named {}\n", file_name.blue());
}

fn print_file(file_name: String) {
	let mut file = std::fs::File::open(&file_name).unwrap();
	let mut contents = String::new();
	file
		.read_to_string(&mut contents)
		.unwrap();
	println!("\n\nContents of file {}: \n\n{}\n", file_name.blue().italic(), contents.truecolor(200,200,200));
}

fn append_to_file(file_name: String, text: String) {
	let mut file = OpenOptions::new()
		.append(true)
		.open(&file_name)
		.expect(&"\nCannot Open File.".red());

	let now = Utc::now().to_string();
	let ftext = "\n".to_owned() + &now + ": " + &text;

	file
		.write_all(ftext.as_bytes())
		.expect(&"\nWrite Failed".red());

	println!("\n\n{}: {}\n", "Successfully appended to file" , file_name.green().bold());
}

struct Tag {
	name: String,
	imp: u32 //importance
}
