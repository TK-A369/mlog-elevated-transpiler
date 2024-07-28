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
            '"' => {
                let mut string_content = String::new();
                let mut str_total_len = 1;
                let mut str_char_iter = char_iter.clone();
                while let Some(str_ch) = str_char_iter.next() {
                    match str_ch {
                        '\\' => {
                            let next_ch = match str_char_iter.next() {
                                Some(some_ch) => some_ch,
                                None => {
                                    return Err(String::from(
                                        "Expected appropriate character after '\\'",
                                    ));
                                }
                            };
                            match next_ch {
                                '\\' => string_content.push('\\'),
                                '\"' => string_content.push('\"'),
                                'n' => string_content.push('\n'),
                                _ => {
                                    return Err(format!(
                                        "Unknown escape sequence \"\\{}\"",
                                        next_ch
                                    ))
                                }
                            }
                            str_total_len += 2;
                        }
                        '\"' => {
                            str_total_len += 1;
                        }
                        other_ch => {
                            string_content.push(other_ch);
                            str_total_len += 1;
                        }
                    }
                }

                for _i in 0..(str_total_len - 1) {
                    char_iter.next();
                }
            }
            number if number.is_digit(10) => {
                //TODO
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
