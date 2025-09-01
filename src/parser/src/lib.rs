use lexer::Token;

#[derive(Debug)]
pub struct AstNode {
    pub kind: String,
    pub children: Vec<AstNode>,
}

/// The full Abstract Syntax Tree (AST)
#[derive(Debug)]
pub struct Ast {
    pub root: AstNode,
}

impl Ast {
    /// Convenience constructor for an empty AST
    pub fn new_root(kind: &str) -> Self {
        Self {
            root: AstNode {
                kind: kind.into(),
                children: vec![],
            },
        }
    }
}

/// The parser entrypoint
pub fn parse(_tokens: Vec<Token>) -> anyhow::Result<Ast> {
    // placeholder implementation for now
    Ok(Ast::new_root("Root"))
}
