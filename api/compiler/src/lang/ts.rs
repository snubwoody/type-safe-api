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

	fn push_fields<I>(&mut self,fields: I)
	where I: IntoIterator<Item = InterfaceField>
	{
		for field in fields.into_iter(){
			self.fields.push(field);
		}
	}

	/// Generate a typescript interface as a string
	fn gen_code(&self) -> String{
		let mut contents = String::new();
		contents.push_str(&format!("interface {}{{\n",self.name));

		for field in &self.fields{
			contents.push_str(&format!("{}",field));
		}
		contents.push_str("}\n\n");

		contents
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

impl std::fmt::Display for InterfaceField{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&format!("\t{}: {},\n",self.name,self._type))
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

impl std::fmt::Display for Type{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		
		match self {
			Self::Number => f.write_str("number"), 
			Self::String => f.write_str("string"), 
			Self::Boolean => f.write_str("boolean"), 
			Self::Array(_type) => f.write_str(&format!("{}[]",_type)), 
		}
	}
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

pub fn codegen(config_path:&str,file_path:&str){
	let contents = fs::read_to_string(config_path).unwrap();
	let schema: ApiSchema = serde_yaml::from_str(&contents).unwrap();
	let mut interfaces = vec![];

	for (_,(key,value)) in schema.structs.iter().enumerate(){
		let mut interface = Interface::new(&key);
		let fields = parse_interface_fields(value);
		interface.push_fields(fields);
		interfaces.push(interface);
	}
	let mut contents = String::new();
	
	for interface in interfaces{
		contents.push_str(&interface.gen_code());
	}
	println!("{:#?}",contents);
	
	fs::write(file_path, contents).unwrap();
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