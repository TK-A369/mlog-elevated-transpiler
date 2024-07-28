use std::collections::BTreeMap;

use crate::tokenizer::*;

struct ProgramAST {
    variables: BTreeMap<String, GlobalVariableAST>,
    functions: BTreeMap<String, FunctionAST>,
}

struct GlobalVariableAST {
    name: String,
}

struct FunctionAST {
    name: String,
}

pub fn parse_program(tokens: &[tokenizer::Token]) -> Result<ProgramAST, String> {
    let mut pos: usize = 0;

    while pos < tokens.len() {
        todo!();
    }
}

fn parse_global_variable(tokens: &[Token], pos: &mut usize) -> Result<GlobalVariableAST, String> {
    if tokens.len() > *pos + 1 {
        match (&tokens[*pos], &tokens[*pos + 1]) {
            (Token::Keyword(Keyword::Let), Token::Identifier(var_name)) => Ok(GlobalVariableAST {
                name: var_name.clone(),
            }),
            _ => Err(String::from("Not a valid global variable definition")),
        }
    } else {
        Err(String::from("Not enough tokens"))
    }
}

fn parse_function(tokens: &[Token], pos: &mut usize) -> Result<FunctionAST, String> {
    //TODO
    todo!();
}
