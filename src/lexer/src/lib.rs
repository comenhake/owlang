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

#[derive(Debug)]
pub struct Token {
    pub kind: String,
    pub lexeme: String,
}

pub fn tokenize(_source: &str) -> anyhow::Result<Vec<Token>> {
    // placeholder
    Ok(vec![])
}
