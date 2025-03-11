use super::{Field, Type};
/// A typescript class
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
}

/// A method on a class
#[derive(Debug,Clone,PartialEq, Eq, PartialOrd, Ord)]
pub struct Method{
	is_async: bool,
	identifier: String,
	parameters: Vec<Field>,
	returns: Option<Type>,
	body: String,
}

/// Builder pattern for [`Method`]
/// 
/// # Example
#[derive(Debug,Clone)]
pub struct MethodBuilder{
	is_async: bool,
	identifier: String,
	parameters: Vec<Field>,
	returns: Option<Type>,
	body: String
}

impl MethodBuilder{
	/// Create a new [`MethodBuilder`]
	/// 
	/// # Example
	/// 
	/// ```
	/// use compiler::ts::{MethodBuilder,Method,Type};
	/// 
	/// let method = MethodBuilder::new("is_player_alive")
	/// 	.is_async()
	/// 	.returns(Type::Boolean)
	/// 	.add_param("player_id",Type::Number)
	/// 	.build();
	/// ```
	pub fn new(identifier: &str) -> Self{
		Self{
			identifier: String::from(identifier),
			is_async: false,
			parameters: vec![],
			returns: None,
			body: String::new()
		}
	}

	/// Mark this method as async
	pub fn is_async(mut self) -> Self{
		self.is_async = true;
		self
	}

	/// Set the method's return type
	pub fn returns(mut self, _type: Type) -> Self{
		self.returns = Some(_type);
		self
	}
	
	/// Add a parameter to the method
	pub fn add_param(mut self, identifier: &str,_type: Type) -> Self{
		self.parameters.push(Field::new(identifier, _type));
		self
	}

	/// Add a parameter to the method
	pub fn body(mut self, body: &str) -> Self{
		self.body = String::from(body);
		self
	}

	/// Build into a [`Method`]
	pub fn build(self) -> Method{
		Method { 
			is_async: self.is_async, 
			identifier: self.identifier.clone(), 
			parameters: self.parameters, 
			returns: self.returns, 
			body: self.body 
		}
	}
}

#[cfg(test)]
mod tests{

	#[test]
	fn method_display(){

	}
}