pub fn generate(ast: &parser::Ast) -> String {
    println!("Generating code from AST: {:?}", ast);
    "// generated code".into()
}
