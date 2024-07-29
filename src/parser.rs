use std::collections::BTreeMap;

use crate::tokenizer::*;

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

enum ProgramASTNode {
    GlobalVariableAST(GlobalVariableAST),
    FunctionAST(FunctionAST),
}

#[derive(Debug)]
pub struct GlobalVariableAST {
    name: String,
}

#[derive(Debug)]
pub struct FunctionAST {
    name: String,
    statements: Vec<StatementASTNode>,
}

#[derive(Debug)]
pub struct LocalVariableAST {
    name: String,
}

#[derive(Debug)]
pub struct AssignmentAST {
    target_var_name: String,
    value: ExpressionASTNode,
}

#[derive(Debug)]
pub enum StatementASTNode {
    LocalVariableAST(LocalVariableAST),
    AssignmentAST(AssignmentAST),
    ExpressionAST(ExpressionASTNode),
}

#[derive(Debug)]
pub struct FunctionCallAST {
    function_name: String,
    args: Vec<ExpressionASTNode>,
}

#[derive(Debug)]
pub enum ExpressionASTNode {
    FunctionCallAST(FunctionCallAST),
    StringLiteral(String),
    NumberLiteral(f64),
}

pub fn parse_program(tokens: &[Token]) -> Result<ProgramAST, String> {
    let mut pos: usize = 0;

    let mut program_ast = ProgramAST::new();

    while pos < tokens.len() {
        let parsing_result = parse_global_variable(tokens, &mut pos)
            .and_then(|x| Ok(ProgramASTNode::GlobalVariableAST(x)))
            .or_else(|_| {
                parse_function(tokens, &mut pos).and_then(|x| Ok(ProgramASTNode::FunctionAST(x)))
            });
        match parsing_result {
            Ok(ProgramASTNode::GlobalVariableAST(global_var)) => {
                program_ast
                    .variables
                    .insert(global_var.name.clone(), global_var);
            }
            Ok(ProgramASTNode::FunctionAST(func)) => {
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
            (Token::Keyword(Keyword::Let), Token::Identifier(var_name)) => {
                *pos += 2;
                Ok(GlobalVariableAST {
                    name: var_name.clone(),
                })
            }
            _ => Err(String::from("Invalid global variable definition")),
        }
    } else {
        Err(String::from("Not enough tokens"))
    }
}

fn parse_function(tokens: &[Token], pos: &mut usize) -> Result<FunctionAST, String> {
    let pos_orig = *pos;
    match (&tokens[*pos], &tokens[*pos + 1], &tokens[*pos + 2]) {
        (
            Token::Keyword(Keyword::Fn),
            Token::Identifier(fn_name),
            Token::Keyword(Keyword::LeftParenthese),
        ) => {
            *pos += 3;
            let mut params = Vec::<String>::new();

            //Parameters
            loop {
                match &tokens[*pos] {
                    Token::Identifier(param_name) => {
                        params.push(param_name.clone());
                        match &tokens[*pos + 1] {
                            Token::Keyword(Keyword::RightParenthese) => {
                                *pos += 2;
                                break;
                            }
                            Token::Keyword(Keyword::Comma) => {
                                *pos += 2;
                            }
                            other => {
                                *pos = pos_orig;
                                return Err(format!(
                                    "Expected either \")\" or \",\", but got \"{:?}\"",
                                    other
                                ));
                            }
                        }
                    }
                    Token::Keyword(Keyword::RightParenthese) => {
                        *pos += 1;
                        break;
                    }
                    other => {
                        *pos = pos_orig;
                        return Err(format!(
                            "Expected either identifier or \")\", but got \"{:?}\"",
                            other
                        ));
                    }
                }
            }

            // Statement block
            match parse_statement_block(tokens, pos) {
                Ok(statements) => Ok(FunctionAST {
                    name: fn_name.clone(),
                    statements,
                }),
                Err(err) => {
                    *pos = pos_orig;
                    return Err(err);
                }
            }
        }
        _ => Err(String::from("Invalid function definition")),
    }
}

fn parse_statement_block(
    tokens: &[Token],
    pos: &mut usize,
) -> Result<Vec<StatementASTNode>, String> {
    let pos_orig = *pos;
    match tokens[*pos] {
        Token::Keyword(Keyword::LeftCurly) => {
            *pos += 1;
            let mut statements = Vec::<StatementASTNode>::new();
            loop {
                if matches!(tokens[*pos], Token::Keyword(Keyword::RightCurly)) {
                    *pos += 1;
                    break;
                }
                let statement_parse_result = parse_statement(tokens, pos);
                match statement_parse_result {
                    Ok(statement) => {
                        statements.push(statement);
                    }
                    Err(err) => {
                        *pos = pos_orig;
                        return Err(err);
                    }
                }
            }
            Ok(statements)
        }
        _ => Err(String::from("Invalid statement block")),
    }
}

fn parse_statement(tokens: &[Token], pos: &mut usize) -> Result<StatementASTNode, String> {
    let parsing_result = parse_local_variable(tokens, pos)
        .and_then(|x| Ok(StatementASTNode::LocalVariableAST(x)))
        .or_else(|_| {
            parse_assignment(tokens, pos).and_then(|x| Ok(StatementASTNode::AssignmentAST(x)))
        })
        .or_else(|_| {
            parse_expression(tokens, pos).and_then(|x| Ok(StatementASTNode::ExpressionAST(x)))
        });
    parsing_result
}

fn parse_local_variable(tokens: &[Token], pos: &mut usize) -> Result<LocalVariableAST, String> {
    if tokens.len() > *pos + 1 {
        match (&tokens[*pos], &tokens[*pos + 1]) {
            (Token::Keyword(Keyword::Let), Token::Identifier(var_name)) => {
                *pos += 2;
                Ok(LocalVariableAST {
                    name: var_name.clone(),
                })
            }
            _ => Err(String::from("Invalid local variable definition")),
        }
    } else {
        Err(String::from("Not enough tokens"))
    }
}

fn parse_assignment(tokens: &[Token], pos: &mut usize) -> Result<AssignmentAST, String> {
    let pos_orig = *pos;
    match (&tokens[*pos], &tokens[*pos + 1]) {
        (Token::Identifier(target_var_name), Token::Keyword(Keyword::Assign)) => {
            *pos += 2;
            todo!();
        }
        _ => Err(String::from("Invalid assignment")),
    }
}

fn parse_expression(tokens: &[Token], pos: &mut usize) -> Result<ExpressionASTNode, String> {
    todo!();
}
