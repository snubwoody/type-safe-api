use std::fs;
use code_generation::generate_code;
use core_types::ApiSchema;


fn main() {
	let contents = fs::read_to_string("../schema.yml").unwrap();
	let data: ApiSchema = serde_yaml::from_str(&contents).unwrap();
	
	generate_code!(data);
	
	println!("{:#?}",data);
}
