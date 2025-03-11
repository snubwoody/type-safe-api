//! Contains all language specific code generation 
pub mod ts;

/// Code generation for rust
pub mod rs{
	pub use code_generation::code_gen;
}
