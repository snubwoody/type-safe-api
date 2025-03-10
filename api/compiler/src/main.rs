use std::fs;
pub use code_generation::generate_code;
use core_types::ApiSchema;

mod api{
	use crate::generate_code;
	generate_code!("../schema.yml");
}

fn main() {
	let contents = fs::read_to_string("../schema.yml").unwrap();
    let schema: ApiSchema = serde_yaml::from_str(&contents).unwrap();
}
