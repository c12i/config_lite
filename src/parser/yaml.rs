use std::path::PathBuf;

use serde::Deserialize;

pub fn parse_yaml<'a, T: Deserialize<'a>>(file_path: &PathBuf, string_path: &str) -> T {
	let file_content = std::fs::read_to_string(file_path).unwrap();
	let mut string_path_vec = string_path.split(".").collect::<Vec<&str>>();
	todo!()
}