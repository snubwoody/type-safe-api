use code_generation::generate_code;
use core_types::ApiSchema;
use std::fs;

fn main() {
    generate_code!("../schema.yml");
}
