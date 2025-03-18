use compiler::ts;

fn main(){
	ts::codegen("examples/schema.yml","../typescript-client/types.ts");
}