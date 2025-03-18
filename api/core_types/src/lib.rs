use proc_macro2::Span;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use quote::{quote, ToTokens, TokenStreamExt};

/// The api schema
/// 
/// # Example schema
/// ```yaml
/// version: 0.1.0
/// schema_diff: none
/// 
/// structs:
///   User:
///     id: string,
///     email: string,
///     phone_number: string
/// ```
#[derive(Debug, Serialize, Deserialize,Clone,PartialEq,Default)]
#[serde(deny_unknown_fields)]
pub struct ApiSchema {
    pub version: String,
    pub schema_diff: String,
    pub structs: HashMap<String, HashMap<String, SchemaType>>,
    pub endpoints: HashMap<String, Endpoint>,
}

impl ApiSchema{
	pub fn parse(contents: &str) -> Result<Self,serde_yaml::Error>{
		let schema:Self = serde_yaml::from_str(contents)?;
		
		Ok(schema)
	}
}

/// A url endpoint
#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
pub struct Endpoint {
	pub uri: String,
    pub method: HttpMethod,
    pub input: SchemaType, // TODO This should be optional
    pub returns: SchemaType,
}

#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SchemaType {
    Int,
	Float,
    String,
	Boolean,
	// Custom struct 
	#[serde(untagged)]
	Struct(String)
}

impl SchemaType{
	/// Parse into a native rust type
	pub fn parse(&self) -> proc_macro2::TokenStream{
		match self {
			Self::Int => quote!{ i32 },
			Self::String => quote!{ String },
			Self::Boolean => quote!{ bool },
			Self::Float => quote!{ f32 },
			Self::Struct(name) => {
				// TODO test this
				let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                quote! { #ident }
			},
		}
	}
}

#[derive(Debug, Serialize, Deserialize,Clone, Copy,PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Delete,
}


impl ToTokens for HttpMethod{
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		match self {
			Self::Get => {
				let ident = proc_macro2::Literal::string("GET",);
				tokens.append(ident);
			},
			Self::Post => {
				let ident = proc_macro2::Literal::string("POST");
				tokens.append(ident);
			},
			Self::Patch => {
				let ident = proc_macro2::Literal::string("PATCH");
				tokens.append(ident);
			},
			Self::Delete => {
				let ident = proc_macro2::Literal::string("DELETE");
				tokens.append(ident);
			},
		}
	}
}