use serde::Deserialize;

pub fn parse_json<'a, T: Deserialize<'a>>(content: String, string_path: &str) -> T {
	let mut string_path_vec = string_path.split(".").collect::<Vec<&str>>();
	let json_value: serde_json::Value = serde_json::from_str(&content).unwrap();
	let mut first_value = {
		if string_path_vec.len() == 1 {
			json_value.get(string_path).unwrap()
		} else {
			json_value.get(string_path_vec.iter().nth(0).unwrap()).unwrap()
		}
	};
	string_path_vec.iter().for_each(|v| {
		json_value.get(v).unwrap();
	});
	todo!()
}