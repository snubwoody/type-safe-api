use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
struct ApiSchema{
	version: String,
	schema_diff: String,
	structs: HashMap<String,HashMap<String,String>>,
	endpoints: HashMap<String,Endpoint>,
}

#[derive(Debug,Serialize,Deserialize)]
struct Endpoint{
	method:String,
	input: String,
	returns: String
}

fn main() {
	let contents = fs::read_to_string("../schema.yml").unwrap();
	let data:ApiSchema = serde_yaml::from_str(&contents).unwrap();
	
	println!("{:#?}",data);
}
