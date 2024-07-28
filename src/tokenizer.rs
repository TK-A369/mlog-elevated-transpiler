#[derive(Debug)]
pub enum Keyword {
    Fn,
    Let,
    LeftCurly,
    RightCurly,
    LeftParenthese,
    RightParenthese,
    Assign,
}

#[derive(Debug)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Number(f64),
    String(String),
}

pub fn tokenize(code: &str) -> Result<Vec<Token>, String> {
    let mut result = Vec::<Token>::new();

    let mut line_counter: usize = 1;
    let mut column_counter: usize = 1;
    //let mut prev_newline_char: Option<char> = None;
    fn handle_newline(line_counter: &mut usize, column_counter: &mut usize) {
        *line_counter += 1;
        *column_counter = 1;
    }

    let mut char_iter = code.chars();
    while let Some(ch) = char_iter.next() {
        match ch {
            letter if letter.is_alphabetic() || matches!(letter, '_' | '@') => {
                //Either keyword or identifier
                let mut identifier = String::new();
                identifier.push(letter);
                let mut ident_len = 1;
                let mut ident_char_iter = char_iter.clone();
                for ident_ch in ident_char_iter {
                    if ident_ch.is_alphanumeric() || ident_ch == '_' {
                        identifier.push(ident_ch);
                    } else {
                        break;
                    }
                    ident_len += 1;
                }

                let token = match identifier.as_str() {
                    "fn" => Token::Keyword(Keyword::Fn),
                    "let" => Token::Keyword(Keyword::Let),
                    _ => Token::Identifier(identifier),
                };
                result.push(token);

                for _i in 0..(ident_len - 1) {
                    char_iter.next();
                }
            }
            '{' => {
                result.push(Token::Keyword(Keyword::LeftCurly));
            }
            '}' => {
                result.push(Token::Keyword(Keyword::RightCurly));
            }
            '(' => {
                result.push(Token::Keyword(Keyword::LeftParenthese));
            }
            ')' => {
                result.push(Token::Keyword(Keyword::RightParenthese));
            }
            '=' => {
                result.push(Token::Keyword(Keyword::Assign));
            }
            ' ' | '\t' => {}
            '\r' => { //Sorry macOS users, no line counting for you
            }
            '\n' => {
                line_counter += 1;
                column_counter = 1;
            }
            _ => {
                return Err(format!(
                    "Unexpected character '{}' at {}:{}",
                    ch, line_counter, column_counter
                ));
            }
        };
        //counter += 1;
    }
    Ok(result)
}
