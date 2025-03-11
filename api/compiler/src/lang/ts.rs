//! Code generation for typescript

use std::{collections::HashMap, fs};
use core_types::{ApiSchema, SchemaType};

/// Represents a typescript interface
#[derive(Debug)]
struct Interface{
	name: String,
	fields: Vec<InterfaceField>
}

impl Interface {
	fn new(name:&str) -> Self{
		// TODO make sure it's a valid identifier
		Self { name: String::from(name), fields: vec![] }
	}

	fn push_field(&mut self,field: InterfaceField){
		self.fields.push(field);
	}
}

#[derive(Debug)]
struct InterfaceField{
	name: String,
	_type: Type 
}

impl InterfaceField {
	fn new(name:&str, _type: Type) -> Self{
		Self { name: String::from(name), _type }
	}
}

#[derive(Debug)]
enum Type{
	/// `number`
	Number,
	/// `string`
	String,
	/// `boolean`
	Boolean,
	/// `T[]`
	Array(Box<Type>)
}


impl From<SchemaType> for Type {
	fn from(value: SchemaType) -> Self {
		match value {
			SchemaType::Int => Self::Number,
			SchemaType::String => Self::String,
		}
	}
}

impl From<&SchemaType> for Type {
	fn from(value: &SchemaType) -> Self {
		match value {
			&SchemaType::Int => Self::Number,
			&SchemaType::String => Self::String,
		}
	}
}

pub fn codegen(path:&str){
	let contents = fs::read_to_string(path).unwrap();
	let schema: ApiSchema = serde_yaml::from_str(&contents).unwrap();
	
	for (_,(key,value)) in schema.structs.iter().enumerate(){
		let interface = Interface::new(&key);
		let fields = parse_interface_fields(value);
		println!("{:#?}",fields);
	}
}

/// Parse typescript interface fields
fn parse_interface_fields(values: &HashMap<String,SchemaType>) -> Vec<InterfaceField>{
	let mut fields = vec![];

	for (_,(key,value)) in values.iter().enumerate(){
		let _type = Type::from(value);
		let field = InterfaceField::new(&key, _type);
		fields.push(field);
	}

	fields
}