use core_types::{Endpoint, HttpMethod};
use proc_macro2::TokenStream;

use super::{Field, TsType};

// TODO add jsdoc
/// A method on a class
#[derive(Debug,Clone,PartialEq, Eq, PartialOrd, Ord)]
pub struct Method{
	is_async: bool,
	identifier: String,
	parameters: Vec<Field>,
	returns: Option<TsType>,
	body: String,
}

impl Method{
	/// Check whether the method is async or not.
	pub fn is_async(&self) -> bool{
		self.is_async
	}
	
	/// Get a reference to the method body.
	pub fn body(&self) -> &str{
		&self.body
	}
	
	pub fn identifier(&self) -> &str{
		&self.identifier
	}

	
	pub fn returns(&self) -> &Option<TsType>{
		&self.returns
	}

	pub fn parameters(&self) -> &[Field]{
		&self.parameters
	}
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
					writeln!(f,": Promise<{}> {{",_type)?;
				}else {
					writeln!(f,": {} {{",_type)?;
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
/// use compiler::ts::{MethodBuilder,Method,TsType};
/// 
/// let method = MethodBuilder::new("is_player_alive")
/// 	.is_async()
/// 	.returns(TsType::Boolean)
/// 	.add_param("player_id",TsType::Number)
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
	returns: Option<TsType>,
	body: String
}

impl MethodBuilder{
	/// Create a new [`MethodBuilder`]
	/// 
	/// # Example
	/// 
	/// ```
	/// use compiler::ts::{MethodBuilder,Method,TsType};
	/// 
	/// let method = MethodBuilder::new("is_player_alive")
	/// 	.is_async()
	/// 	.returns(TsType::Boolean)
	/// 	.add_param("player_id",TsType::Number)
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
	pub fn returns(mut self, _type: TsType) -> Self{
		self.returns = Some(_type);
		self
	}
	
	/// Add a parameter to the method
	pub fn add_param(mut self, identifier: &str,_type: TsType) -> Self{
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
	pub fn from_endpoint(name:&str,endpoint: &Endpoint) -> Method{
		let input_type:TsType = endpoint.input.clone().into();
		let return_type:TsType = endpoint.returns.clone().into();
		let http_method = endpoint.method;
		let uri = endpoint.uri.clone();
		
		let method_body = quote::quote!{
			try{
				const response = await fetch(#uri,{
					headers:{
						"Api-Checksum": this.checksum
					},
					method: #http_method,
					body: JSON.stringify(payload)
				});
				if (response.ok){
					const body: #return_type = await response.json();
					return body;
				}
				else{
					const error = await response.json();
					throw error;
				}
			} catch (err){
				throw err;
			}
		};
		
		
		
		MethodBuilder::new(name)
			.add_param("payload", input_type)
			.returns(return_type)
			.is_async()
			.body(&method_body.to_string())
			.build()	
	}
}

impl MethodBuilder{
	
	fn gen_get_method(endpoint: &Endpoint) -> TokenStream{
		let return_type:TsType = endpoint.returns.clone().into();
		let uri = endpoint.uri.clone();
		
		quote::quote!{
			try{
				const response = await fetch(#uri,{
					headers:{
						"Api-Checksum": this.checksum
					}
				});
				if (response.ok){
					const body: #return_type = await response.json();
					return body;
				}
				else{
					const error = await response.json();
					throw error;
				}
			} catch (err){
				throw err;
			}
		}
	}

	fn gen_post_method(endpoint: &Endpoint) -> TokenStream{
		let input_type:TsType = endpoint.input.clone().into();
		let return_type:TsType = endpoint.returns.clone().into();
		let uri = endpoint.uri.clone();
		
		quote::quote!{
			try{
				const response = await fetch(#uri,{
					headers:{
						"Api-Checksum": this.checksum
					},
					method: "POST"
				});
				if (response.ok){
					const body: #return_type = await response.json();
					return body;
				}
				else{
					const error = await response.json();
					throw error;
				}
			} catch (err){
				throw err;
			}
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
			.add_param("uid", TsType::Number)
			.returns(TsType::Custom("User".to_string()))
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