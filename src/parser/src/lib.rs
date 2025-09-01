// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

use lexer::Token;

#[derive(Debug)]
pub struct AstNode {
    pub kind: String,
    pub children: Vec<AstNode>,
}

pub fn parse(_tokens: Vec<Token>) -> anyhow::Result<AstNode> {
    // placeholder
    Ok(AstNode {
        kind: "Root".into(),
        children: vec![],
    })
}
