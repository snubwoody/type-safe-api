use proc_macro2::Span;
use quote::quote;
use std::fs;
use core_types::ApiSchema;

/// Generate code from a schema file
#[proc_macro]
pub fn generate_code(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = input.to_string().replace("\"", "");
	
	let contents = fs::read_to_string(path).unwrap();
    let schema: ApiSchema = serde_yaml::from_str(&contents).unwrap();

	let structs = parse_structs(&schema);

	println!("{:#?}",structs);


    let expanded = quote! {
		#structs
    };

    proc_macro::TokenStream::from(expanded)
}

fn parse_structs(schema:&ApiSchema) -> proc_macro2::TokenStream{
	let mut structs = vec![];
	
	for (_,(key,value)) in schema.structs.iter().enumerate(){
		println!("{:#?}{:#?}",key,value);
		let struct_name = syn::Ident::new(&key, Span::call_site());
		
		let _struct = quote! {
			struct #struct_name{

			}
		};

		structs.push(_struct);
	}

	quote! {#(#structs)*}
}
