//! Code generation for typescript
//! 
//! # Example 
//! ```yaml
//! struct User{
//! 	id: int,
//! 	name: string,
//! 	email: string,
//! 	created_at: string,
//! }
//! ```
//! 
//! output:
//! 
//! ```typescript
//! interface User{
//! 	id: number,
//! 	name: string,
//! 	email: string,
//! 	created_at: string
//! }
//! ```
use std::{collections::HashMap, fs};
use core_types::{ApiSchema, SchemaType};
mod class;
mod method;
pub use method::*;
pub use class::*;
use proc_macro2::Span;
use quote::{quote, ToTokens, TokenStreamExt};

/// Represents a typescript interface
/// 
/// ```typescript
/// interface User{
/// 	id: string,
/// 	email: string,
/// 	name?: string,
/// 	createdAt: string,
/// }
/// ```
#[derive(Debug)]
struct Interface{
	name: String,
	fields: Vec<Field>
}

impl Interface {
	fn new(name:&str) -> Self{
		// TODO make sure it's a valid identifier
		Self { name: String::from(name), fields: vec![] }
	}

	fn push_field(&mut self,field: Field){
		self.fields.push(field);
	}

	fn push_fields<I>(&mut self,fields: I)
	where I: IntoIterator<Item = Field>
	{
		for field in fields.into_iter(){
			self.fields.push(field);
		}
	}

	/// Generate a typescript interface as a string
	fn gen_code(&self) -> String{
		let mut contents = String::new();
		contents.push_str(&format!("export interface {}{{\n",self.name));

		for field in &self.fields{
			contents.push_str(&format!("\t{},\n",field));
		}
		contents.push_str("}\n\n");

		contents
	}
}

/// A typescript interface or class field
#[derive(Debug,Clone,PartialEq, Eq, PartialOrd, Ord)]
pub struct Field{
	name: String,
	_type: TsType 
}

impl Field {
	fn new(name:&str, _type: TsType) -> Self{
		Self { name: String::from(name), _type }
	}
}

impl std::fmt::Display for Field{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&format!("{}: {}",self.name,self._type))
	}
}

/// A typescript type
#[derive(Debug,Clone,PartialEq, Eq, PartialOrd, Ord)]
pub enum TsType{
	/// `number`
	Number,
	/// `string`
	String,
	/// `boolean`
	Boolean,
	/// `T[]`
	Array(Box<TsType>),
	Custom(String),
}

impl std::fmt::Display for TsType{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Number => f.write_str("number"), 
			Self::String => f.write_str("string"), 
			Self::Boolean => f.write_str("boolean"), 
			Self::Array(_type) => f.write_str(&format!("{}[]",_type)), 
			Self::Custom(_type) => f.write_str(&_type.to_string()), 
		}
	}
}

impl From<SchemaType> for TsType {
	fn from(value: SchemaType) -> Self {
		match value {
			SchemaType::Int | 
			SchemaType::Float => Self::Number,
			SchemaType::String => Self::String,
			SchemaType::Boolean => Self::Boolean,
			SchemaType::Struct(ident) => Self::Custom(ident)
		}
	}
}

impl From<&SchemaType> for TsType {
	fn from(value: &SchemaType) -> Self {
		match value {
			&SchemaType::Int|
			&SchemaType::Float => Self::Number,
			&SchemaType::String => Self::String,
			&SchemaType::Boolean => Self::Boolean,
			SchemaType::Struct(ident) => Self::Custom(ident.clone())
		}
	}
}


impl ToTokens for TsType{
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		match self {
			Self::Number => {
				let ident = proc_macro2::Ident::new("number", Span::call_site());
				tokens.append(ident);
			}
			Self::String => {
				let ident = proc_macro2::Ident::new("string", Span::call_site());
				tokens.append(ident);
			}
			Self::Array(name) => {
				let ident = proc_macro2::Ident::new(&format!("{}[]",name), Span::call_site());
				tokens.append(ident);
			}
			Self::Boolean => {
				let ident = proc_macro2::Ident::new("boolean", Span::call_site());
				tokens.append(ident);
			}
			Self::Custom(name) => {
				let ident = proc_macro2::Ident::new(name, Span::call_site());
				tokens.append(ident);
			}
		}
	}
}

// TODO check if the custom structs are in the schema
pub fn codegen(config_path:&str,file_path:&str) -> crate::Result<()>{
	let contents = fs::read_to_string(config_path)?;
	let schema = ApiSchema::parse(&contents)?;

	let mut interfaces = vec![];

	for (key,value) in schema.structs.iter(){
		let mut interface = Interface::new(key);
		let fields = parse_interface_fields(value);
		interface.push_fields(fields);
		interfaces.push(interface);
	}
	let mut contents = String::new();
	
	for interface in interfaces{
		contents.push_str(&interface.gen_code());
	}

	// Create client
	let mut client = Class::new("Client");
	client.push_field(Field::new("checksum", TsType::String));

	let constructor = MethodBuilder::new("constructor")
		.add_param("checksum", TsType::String)
		.body(
			&quote! {
				this.checksum = checksum;
			}.to_string()
		)
		.build();
	
	client.push_method(constructor);

	// Create route endpoint functions 
	for (name,endpoint) in schema.endpoints{
		let method = MethodBuilder::from_endpoint(&name, &endpoint);
		
		client.push_method(method);
	}


	contents.push_str(&format!("{}",client));
	
	fs::write(file_path, contents)?;

	Ok(())
}

/// Parse typescript interface fields
fn parse_interface_fields(values: &HashMap<String,SchemaType>) -> Vec<Field>{
	let mut fields = vec![];

	for (key,value) in values.iter(){
		let _type = TsType::from(value);
		let field = Field::new(key, _type);
		fields.push(field);
	}

	fields
}

#[cfg(test)]
mod tests{
    use super::*;

	#[test]
	fn type_display(){
		assert_eq!(format!("{}",TsType::Number),"number");
		assert_eq!(format!("{}",TsType::String),"string");
		assert_eq!(format!("{}",TsType::Boolean),"boolean");

		let number_array = TsType::Array(Box::new(TsType::Number));
		assert_eq!(format!("{}",number_array),"number[]");
	}
}