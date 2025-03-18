use compiler::ts::MethodBuilder;
use core_types::{Endpoint, HttpMethod, SchemaType};
use quote::quote;

#[test]
fn get_method_code_gen(){
	let endpoint = Endpoint{
		uri: "https://youtube.com/user".to_owned(),
		method: HttpMethod::Get,
		input: SchemaType::Boolean,
		returns: SchemaType::String
	};

	let method = MethodBuilder::from_endpoint("get_user", endpoint);

	let body = quote! {
		try{
			const response = await fetch("https://youtube.com/user");
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

	assert_eq!(method.body(),&body);
}