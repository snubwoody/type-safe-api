use serde::{Serialize,Deserialize};
use std::collections::HashMap;


#[derive(Debug,Serialize,Deserialize)]
pub struct ApiSchema{
	version: String,
	schema_diff: String,
	structs: HashMap<String,HashMap<String,SchemaType>>,
	endpoints: HashMap<String,Endpoint>,
}

/// A url endpoint
#[derive(Debug,Serialize,Deserialize)]
pub struct Endpoint{
	method: HttpMethod,
	input: String,
	returns: String
}


#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="lowercase")]
pub enum SchemaType{
	Int,
	String,
	DateTime,
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="UPPERCASE")]
pub enum HttpMethod{
	Get,
	Post,
	Patch,
	Delete
}

