use compiler::ts::codegen;

fn main(){
	codegen("examples/schema.yml","../types.d.ts");
}