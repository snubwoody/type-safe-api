use compiler::ts;

fn main() -> Result<(),compiler::Error>{
	ts::codegen(
		"examples/schema.yml",
		"../typescript-client/types.ts"
	)?;
	
	Ok(())
}