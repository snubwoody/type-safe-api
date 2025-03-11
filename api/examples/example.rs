use compiler::ts::codegen;

fn main(){
	codegen("examples/schema.yml","../typescript-client/types.ts");
}