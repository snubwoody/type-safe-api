use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
struct ApiSchema{
	version: String,
	schema_diff: String,
	structs: HashMap<String,HashMap<String,SchemaType>>,
	endpoints: HashMap<String,Endpoint>,
}

/// A url endpoint
#[derive(Debug,Serialize,Deserialize)]
struct Endpoint{
	method: HttpMethod,
	input: String,
	returns: String
}


#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="lowercase")]
enum SchemaType{
	Int,
	String,
	DateTime,
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="UPPERCASE")]
enum HttpMethod{
	Get,
	Post,
	Patch,
	Delete
}

fn main() {
	let contents = fs::read_to_string("../schema.yml").unwrap();
	let data:ApiSchema = serde_yaml::from_str(&contents).unwrap();
	
	println!("{:#?}",data);
}
