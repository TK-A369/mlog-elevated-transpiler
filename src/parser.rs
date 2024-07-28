use std::collections::BTreeMap;

use crate::tokenizer::*;

trait ParseableTrait {
    fn parse(&self, tokens: &[Token], pos: &mut usize) -> Result<Box<dyn ASTTrait>, String>;
}

struct Parseable<ParsedT> {
    parse_fn: fn(&[Token], &mut usize) -> Result<ParsedT, String>,
}

impl<ParsedT: ASTTrait + 'static> ParseableTrait for Parseable<ParsedT> {
    fn parse(&self, tokens: &[Token], pos: &mut usize) -> Result<Box<dyn ASTTrait>, String> {
        match (self.parse_fn)(tokens, pos) {
            Ok(parsed) => Ok(Box::new(parsed)),
            Err(err) => Err(err),
        }
    }
}

/*impl<ParsedT> Parseable<ParsedT> {
    type FnT = fn(&[Token], &mut usize) -> Result<ParsedT, String>;
}*/

fn try_parseables(
    tokens: &[Token],
    pos: &mut usize,
    parseables: &[Box<dyn ParseableTrait>],
) -> Result<Box<dyn ASTTrait>, String> {
    let mut errors = Vec::<String>::new();
    for parseable in parseables {
        match parseable.parse(tokens, pos) {
            Ok(result) => {
                return Ok(result);
            }
            Err(err) => {
                errors.push(err);
            }
        }
    }
    let mut err_msg = String::from("None of parseables worked");
    for e in errors {
        err_msg.push('\n');
        err_msg.push_str(&e);
    }
    Err(err_msg)
}

trait ASTTrait {}

struct ProgramAST {
    variables: BTreeMap<String, GlobalVariableAST>,
    functions: BTreeMap<String, FunctionAST>,
}
impl ASTTrait for ProgramAST {}

struct GlobalVariableAST {
    name: String,
}
impl ASTTrait for GlobalVariableAST {}

struct FunctionAST {
    name: String,
}
impl ASTTrait for FunctionAST {}

pub fn parse_program(tokens: &[Token]) -> Result<ProgramAST, String> {
    let mut pos: usize = 0;

    while pos < tokens.len() {
        todo!();
    }
    Err(String::from("TODO"))
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
