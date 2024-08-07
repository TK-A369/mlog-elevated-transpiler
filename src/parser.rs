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
enum FunctionStyle {
    Normal,
    Inline,
}

#[derive(Debug)]
pub struct FunctionAST {
    name: String,
    pub(crate) statements: Vec<StatementASTNode>,
    style: FunctionStyle,
}

#[derive(Clone, Debug)]
pub struct LocalVariableAST {
    pub(crate) name: String,
}

#[derive(Debug)]
pub struct AssignmentAST {
    pub(crate) target_var_name: String,
    pub(crate) value: ExpressionASTNode,
}

#[derive(Debug)]
pub struct IfAST {
    pub(crate) condition: ExpressionASTNode,
    pub(crate) then_block: Vec<StatementASTNode>,
    pub(crate) else_block: Vec<StatementASTNode>,
}

#[derive(Debug)]
pub struct WhileAST {
    pub(crate) condition: ExpressionASTNode,
    pub(crate) do_block: Vec<StatementASTNode>,
}

#[derive(Debug)]
pub enum StatementASTNode {
    LocalVariableAST(LocalVariableAST),
    AssignmentAST(AssignmentAST),
    ExpressionAST(ExpressionASTNode),
    IfAST(IfAST),
    WhileAST(WhileAST),
}

#[derive(Clone, Debug)]
pub struct FunctionCallAST {
    pub(crate) function_name: String,
    pub(crate) args: Vec<ExpressionASTNode>,
}

#[derive(Clone, Debug)]
pub enum ExpressionASTNode {
    FunctionCallAST(FunctionCallAST),
    StringLiteral(String),
    NumberLiteral(f64),
    VariableReference(String),
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
    let mut style = FunctionStyle::Normal;
    if matches!(&tokens[*pos], Token::Keyword(Keyword::Inline)) {
        *pos += 1;
        style = FunctionStyle::Inline;
    }
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
                    style,
                }),
                Err(err) => {
                    *pos = pos_orig;
                    return Err(err);
                }
            }
        }
        _ => {
            *pos = pos_orig;
            Err(String::from("Invalid function definition"))
        }
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
    let pos_orig = *pos;
    let parsing_result = parse_local_variable(tokens, pos)
        .and_then(|x| Ok(StatementASTNode::LocalVariableAST(x)))
        .or_else(|_| {
            parse_assignment(tokens, pos).and_then(|x| Ok(StatementASTNode::AssignmentAST(x)))
        })
        .or_else(|_| {
            parse_expression(tokens, pos).and_then(|x| Ok(StatementASTNode::ExpressionAST(x)))
        })
        .or_else(|_| parse_if(tokens, pos).and_then(|x| Ok(StatementASTNode::IfAST(x))))
        .or_else(|_| parse_while(tokens, pos).and_then(|x| Ok(StatementASTNode::WhileAST(x))));
    match parsing_result {
        Ok(statement) => {
            println!("parse_statement ok @ token {}", pos_orig);
            Ok(statement)
        }
        Err(_) => Err(String::from("Invalid statement")),
    }
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
            match parse_expression(tokens, pos) {
                Ok(expression) => {
                    println!("parse_assignment ok @ token {}", pos_orig);
                    Ok(AssignmentAST {
                        target_var_name: target_var_name.clone(),
                        value: expression,
                    })
                }
                Err(err) => {
                    *pos = pos_orig;
                    Err(err)
                }
            }
        }
        _ => Err(String::from("Invalid assignment")),
    }
}

fn parse_expression(tokens: &[Token], pos: &mut usize) -> Result<ExpressionASTNode, String> {
    let parsing_result = parse_function_call(tokens, pos)
        .and_then(|x| {
            println!("parse_expression -> parse_function_call ok");
            Ok(ExpressionASTNode::FunctionCallAST(x))
        })
        .or_else(|_| {
            parse_string_literal(tokens, pos).and_then(|x| Ok(ExpressionASTNode::StringLiteral(x)))
        })
        .or_else(|_| {
            parse_number_literal(tokens, pos).and_then(|x| Ok(ExpressionASTNode::NumberLiteral(x)))
        })
        .or_else(|_| {
            parse_variable_reference(tokens, pos)
                .and_then(|x| Ok(ExpressionASTNode::VariableReference(x)))
        });

    match parsing_result {
        Ok(expr) => Ok(expr),
        Err(_) => Err(String::from("Invalid expression")),
    }
}

fn parse_function_call(tokens: &[Token], pos: &mut usize) -> Result<FunctionCallAST, String> {
    let pos_orig = *pos;

    match (&tokens[*pos], &tokens[*pos + 1]) {
        (Token::Identifier(fn_name), Token::Keyword(Keyword::LeftParenthese)) => {
            *pos += 2;

            //Arguments
            let mut args = Vec::<ExpressionASTNode>::new();
            loop {
                if matches!(&tokens[*pos], Token::Keyword(Keyword::RightParenthese)) {
                    *pos += 1;
                    break;
                }

                let expr_parse_result = parse_expression(tokens, pos);
                match expr_parse_result {
                    Ok(arg_expr) => {
                        args.push(arg_expr);
                    }
                    Err(err) => {
                        *pos = pos_orig;
                        return Err(err);
                    }
                }

                if matches!(&tokens[*pos], Token::Keyword(Keyword::RightParenthese)) {
                    *pos += 1;
                    break;
                }
                if matches!(&tokens[*pos], Token::Keyword(Keyword::Comma)) {
                    *pos += 1;
                }
            }

            println!("parse_function_call ok @ token {}", pos_orig);
            Ok(FunctionCallAST {
                function_name: fn_name.clone(),
                args,
            })
        }
        _ => Err(String::from("Invalid function call")),
    }
}

fn parse_string_literal(tokens: &[Token], pos: &mut usize) -> Result<String, String> {
    match &tokens[*pos] {
        Token::String(str) => {
            *pos += 1;
            Ok(str.clone())
        }
        _ => Err(String::from("Invalid string literal")),
    }
}

fn parse_number_literal(tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
    match &tokens[*pos] {
        Token::Number(num) => {
            *pos += 1;
            Ok(*num)
        }
        _ => Err(String::from("Invalid number literal")),
    }
}

fn parse_variable_reference(tokens: &[Token], pos: &mut usize) -> Result<String, String> {
    match &tokens[*pos] {
        Token::Identifier(ident) => {
            *pos += 1;
            Ok(ident.clone())
        }
        _ => Err(String::from("Invalid variable reference")),
    }
}

fn parse_if(tokens: &[Token], pos: &mut usize) -> Result<IfAST, String> {
    let pos_orig = *pos;
    match &tokens[*pos] {
        Token::Keyword(Keyword::If) => {
            *pos += 1;

            //Condition
            match parse_expression(tokens, pos) {
                Ok(condition_expr) => {
                    //Then block
                    match parse_statement_block(tokens, pos) {
                        Ok(then_block) => match &tokens[*pos] {
                            Token::Keyword(Keyword::Else) => {
                                *pos += 1;

                                //Else block
                                match parse_statement_block(tokens, pos) {
                                    Ok(else_block) => Ok(IfAST {
                                        condition: condition_expr,
                                        then_block,
                                        else_block,
                                    }),
                                    Err(err) => {
                                        *pos = pos_orig;
                                        Err(err)
                                    }
                                }
                            }
                            _ => Ok(IfAST {
                                condition: condition_expr,
                                then_block,
                                else_block: Vec::new(),
                            }),
                        },
                        Err(err) => {
                            *pos = pos_orig;
                            Err(err)
                        }
                    }
                }
                Err(err) => {
                    *pos = pos_orig;
                    Err(err)
                }
            }
        }
        _ => Err(String::from("Invalid if statement")),
    }
}

fn parse_while(tokens: &[Token], pos: &mut usize) -> Result<WhileAST, String> {
    let pos_orig = *pos;
    match &tokens[*pos] {
        Token::Keyword(Keyword::While) => {
            *pos += 1;

            //Condition
            match parse_expression(tokens, pos) {
                Ok(condition_expr) => {
                    //Do block
                    match parse_statement_block(tokens, pos) {
                        Ok(do_block) => Ok(WhileAST {
                            condition: condition_expr,
                            do_block,
                        }),
                        Err(err) => {
                            *pos = pos_orig;
                            Err(err)
                        }
                    }
                }
                Err(err) => {
                    *pos = pos_orig;
                    Err(err)
                }
            }
        }
        _ => Err(String::from("Invalid while statement")),
    }
}
