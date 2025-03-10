use quote::quote;
use std::fs;
use core_types::ApiSchema;

/// Generate code from a schema file
#[proc_macro]
pub fn generate_code(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = input.to_string().replace("\"", "");
	
	let contents = fs::read_to_string(path).unwrap();
    let data: ApiSchema = serde_yaml::from_str(&contents).unwrap();

    println!("{:#?}", data);

    let expanded = quote! {
        println!("Hello world")
    };

    proc_macro::TokenStream::from(expanded)
}
