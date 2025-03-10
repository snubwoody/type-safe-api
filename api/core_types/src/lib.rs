use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use quote::quote;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiSchema {
    pub version: String,
    pub schema_diff: String,
    pub structs: HashMap<String, HashMap<String, SchemaType>>,
    pub endpoints: HashMap<String, Endpoint>,
}

/// A url endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct Endpoint {
    pub method: HttpMethod,
    pub input: String,
    pub returns: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SchemaType {
    Int,
    String,
}

impl SchemaType{
	/// Parse into a native rust type
	pub fn parse(&self) -> proc_macro2::TokenStream{
		match &self {
			&Self::Int => quote!{ i32 },
			&Self::String => quote!{ String },
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Delete,
}
