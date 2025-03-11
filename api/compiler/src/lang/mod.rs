//! Contains all language specific code generation 

/// Code generation for rust
pub mod rs{
	pub use code_generation::code_gen;
}

/// Code generation for typescript
pub mod ts{
    use std::fs;
    use core_types::ApiSchema;

	pub fn typescript_codegen(path:&str){
		let contents = fs::read_to_string(path).unwrap();
    	let schema: ApiSchema = serde_yaml::from_str(&contents).unwrap();
		println!("{:#?}",schema);
	}
}