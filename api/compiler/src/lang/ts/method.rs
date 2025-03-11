use super::{Field, Type};

/// A method on a class
#[derive(Debug,Clone,PartialEq, Eq, PartialOrd, Ord)]
pub struct Method{
	is_async: bool,
	identifier: String,
	parameters: Vec<Field>,
	returns: Option<Type>,
	body: String,
}

impl std::fmt::Display for Method{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.is_async{
			write!(f,"async ")?;
		}
		write!(f,"{}",self.identifier)?;
		write!(f,"(")?;
		
		for param in &self.parameters{
			write!(f,"{},",param)?;
		}
		
		write!(f,")")?;
		match &self.returns {
			Some(_type) => {
				if self.is_async{
					write!(f,": Promise<{}> {{\n",_type)?;
				}else {
					write!(f,": {} {{\n",_type)?;
				}
			},
			None =>{
				write!(f," {{")?
			}
		}
		
		write!(f,"{}",self.body)?;
		write!(f,"\n}}")
	}
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
	/// 
	/// The generated typescript code would be 
	/// 
	/// ```typescript
	/// // Dummy class 
	/// class World{
	/// 	async is_player_alive(player_id: number): Promise<boolean>{
	/// 
	/// 	}
	/// }
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
	use super::*;

	#[test]
	fn async_method(){
		let method = MethodBuilder::new("get_user")
			.is_async()
			.add_param("uid", Type::Number)
			.returns(Type::Custom("User".to_string()))
			.body("")
			.build();

		let body = concat!("async get_user(uid: number,): Promise<User> {","\n","\n}");
		
		assert_eq!(format!("{}",method),body);
	}

	#[test]
	fn sync_method(){
		let method = MethodBuilder::new("init")
			.body("")
			.build();

		let body = concat!("init() {","\n}");
		
		assert_eq!(format!("{}",method),body);
	}
}