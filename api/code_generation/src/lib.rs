use proc_macro2::Span;
use quote::quote;
use std::{collections::HashMap, fs};
use core_types::{ApiSchema, SchemaType};

/// Generate code from a schema file
/// ```yaml
/// structs:
///   User:
///     id: int
///     name: string
///     email: string
/// ```
/// Generated code:
/// ```
/// struct User{
/// 	id: i32,
/// 	name: String,
/// 	email: String
/// }
/// ```
#[proc_macro]
pub fn code_gen(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = input.to_string().replace("\"", "");
	
	let contents = fs::read_to_string(path).unwrap();
    let schema: ApiSchema = serde_yaml::from_str(&contents).unwrap();

	let structs = parse_structs(&schema);

    let expanded = quote! {
		#structs
    };

    proc_macro::TokenStream::from(expanded)
}

fn parse_structs(schema:&ApiSchema) -> proc_macro2::TokenStream{
	let mut structs = vec![];
	
	for (_,(key,value)) in schema.structs.iter().enumerate(){
		let struct_name = syn::Ident::new(&key, Span::call_site());
		
		let struct_fields = parse_struct_fields(value);

		let _struct = quote! {
			pub struct #struct_name{
				#struct_fields
			}
		};

		structs.push(_struct);
	}

	quote! {#(#structs)*}
}

fn parse_struct_fields(fields: &HashMap<String,SchemaType>) -> proc_macro2::TokenStream{
	let mut struct_fields = vec![];
	for (_,(key,value)) in fields.iter().enumerate(){
		let field_name = syn::Ident::new(&key, Span::call_site());	
		let field_type = value.parse();	
		
		let field = quote! {
			pub #field_name: #field_type
		};

		struct_fields.push(field);
	}

	quote! {#(#struct_fields),*}
}
