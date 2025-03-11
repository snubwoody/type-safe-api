use super::{Field, Type,Method};

/// A typescript class
#[derive(Debug,Clone,PartialEq, Eq, PartialOrd, Ord)]
pub struct Class{
	name: String,
	fields: Vec<Field>,
	methods: Vec<Method>
}

impl Class{
	pub fn new(name:&str) -> Self{
		Self { 
			name: String::from(name), 
			fields: vec![], 
			methods: vec![] 
		}
	}

	pub fn push_field(&mut self,field: Field){
		self.fields.push(field);
	}

	/// Add a method to the class
	pub fn push_method(&mut self, method: Method){
		self.methods.push(method);
	}
}

impl std::fmt::Display for Class{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"class {} {{\n",self.name)?;

		for field in &self.fields{
			write!(f,"\t{}\n",field)?;
		}

		for method in &self.methods{
			write!(f,"\t{}\n",method)?;
		}

		write!(f,"}}")
	}
}

#[cfg(test)]
mod tests{
	use crate::ts::MethodBuilder;
	use super::*;

	#[test]
	fn class_display(){
		let mut class = Class::new("User");
		class.push_field(Field::new("id", Type::Number));
		class.push_field(Field::new("email", Type::String));
		class.push_field(Field::new("user_name", Type::String));

		let id = MethodBuilder::new("id")
			.returns(Type::Number)
			.build();
		
		let username = MethodBuilder::new("user_name")
			.returns(Type::String)
			.build();

		class.push_method(id);
		class.push_method(username);

		let output = concat!(
			"class User {\n",
			"\tid: number\n",
			"\temail: string\n",
			"\tuser_name: string\n",
			"\tid(): number {n\n",
			"\n",
			"}",
		);
		println!("{}",&class);
	}
}