use core_types::Endpoint;
use quote::quote;

use super::{Field, Type};

// TODO add jsdoc
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
		
		write!(f,"\t\t{}",self.body)?;
		write!(f,"\n\t}}")
	}
}

/// Builder pattern for [`Method`]
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
	
	/// Create a new method from an [`Endpoint`].
	pub fn from_endpoint(name:&str,endpoint: Endpoint) -> Method{
		let param_type:Type = endpoint.input.into();
		let return_type:Type = endpoint.returns.into();

		let mut method_body = format!(
			r#"try {{
			const response = await fetch('{}');
			if (response.ok){{
				const user: {} = await response.json();
				return user; 
			}} 
			else{{
				// throw error body
				const error = response.json();
				throw error;
			}}
			}} catch(err) {{
				throw err;
			}}"#,
			endpoint.uri,
			return_type
		);

		let uri = endpoint.uri;

		let method_body = quote::quote!{
			try{
				const response = await fetch(#uri);
				if (response.ok){
					const user: User = await response.json();
					return user;
				}
				else{
					const error = response.json();
					throw error;
				}
			} catch (err){
				throw err;
			}
		};

		let method = MethodBuilder::new(&name)
			.add_param("body", param_type)
			.returns(return_type)
			.is_async()
			.body(&method_body.to_string())
			.build();

		method	
	}
}

#[cfg(test)]
mod tests{
	use core_types::{HttpMethod, SchemaType};
	use super::*;

	#[test]
	fn get_method_gen(){
		let endpoint = Endpoint{
			uri: "https::/youtube.com/user".to_owned(),
			method: HttpMethod::Get,
			input: SchemaType::Boolean,
			returns: SchemaType::String
		};

		let method = MethodBuilder::from_endpoint("get_user", endpoint);

		let body = quote! {
			try{
				const response = await fetch("https::/youtube.com/user");
				if (response.ok){
					const body: string = await response.json();
					return body;
				}else{
					const error = await response.json();
					throw error;
				}
			} catch (err){
				throw err;
			}
		}.to_string();

		println!("{:#?}",method.body);
		assert_eq!(method.body,body);
	}

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