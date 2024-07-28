use std::collections::BTreeMap;

use crate::tokenizer::*;

enum ASTNode {
    ProgramAST(ProgramAST),
    GlobalVariableAST(GlobalVariableAST),
    FunctionAST(FunctionAST),
}

#[derive(Debug)]
pub struct ProgramAST {
    variables: BTreeMap<String, GlobalVariableAST>,
    functions: BTreeMap<String, FunctionAST>,
}
impl ProgramAST {
    fn new() -> Self {
        Self {
            variables: BTreeMap::new(),
            functions: BTreeMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct GlobalVariableAST {
    name: String,
}

#[derive(Debug)]
pub struct FunctionAST {
    name: String,
}

pub fn parse_program(tokens: &[Token]) -> Result<ProgramAST, String> {
    let mut pos: usize = 0;

    let mut program_ast = ProgramAST::new();

    while pos < tokens.len() {
        let parsing_result = parse_global_variable(tokens, &mut pos)
            .and_then(|x| Ok(ASTNode::GlobalVariableAST(x)))
            .or_else(|_| {
                parse_function(tokens, &mut pos).and_then(|x| Ok(ASTNode::FunctionAST(x)))
            });
        match parsing_result {
            Ok(ASTNode::GlobalVariableAST(global_var)) => {
                program_ast
                    .variables
                    .insert(global_var.name.clone(), global_var);
            }
            Ok(ASTNode::FunctionAST(func)) => {
                program_ast.functions.insert(func.name.clone(), func);
            }
            Ok(_) => unreachable!(),
            Err(err) => return Err(err),
        }
    }
    Ok(program_ast)
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
