#![allow(unused)]

use std::fs;

mod preprocessor;
// mod lexer;

pub fn compile(input_files: Vec<String>, options: Vec<String>, flags: String) -> Result<(), std::io::Error> {
	for filename in input_files {
		let file = fs::read_to_string(filename)?;
		let pp_file = preprocessor::preprocessor(file);
	}
	
	Ok(())
}
