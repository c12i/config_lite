use serde::Deserialize;

pub fn parse_yaml<'a, T: Deserialize<'a>>(content: String, string_path: &str) -> T {
	todo!()
}