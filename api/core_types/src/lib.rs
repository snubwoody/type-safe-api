use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Delete,
}
