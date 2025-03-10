use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn generate_code(input:proc_macro::TokenStream) -> proc_macro::TokenStream{
	let expanded = quote! {
		println!("Hello world")
	};

	proc_macro::TokenStream::from(expanded)
}	