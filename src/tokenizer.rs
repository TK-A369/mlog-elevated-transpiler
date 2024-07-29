#[derive(Debug)]
pub enum Keyword {
    Fn,
    Let,
    If,
    Else,
    While,
    LeftCurly,
    RightCurly,
    LeftParenthese,
    RightParenthese,
    Assign,
    Comma,
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

    let mut char_iter = code.chars().peekable();
    while let Some(ch) = char_iter.peek() {
        println!("Current character: \'{}\'", ch);
        match ch {
            letter if letter.is_alphabetic() || matches!(letter, '_' | '@') => {
                //Either keyword or identifier
                let mut identifier = String::new();
                identifier.push(*letter);
                char_iter.next();
                let mut ident_len = 1;
                while let Some(ident_ch) = char_iter.peek() {
                    if ident_ch.is_alphanumeric() || *ident_ch == '_' {
                        identifier.push(*ident_ch);
                        char_iter.next();
                        ident_len += 1;
                    } else {
                        break;
                    }
                }

                let token = match identifier.as_str() {
                    "fn" => Token::Keyword(Keyword::Fn),
                    "let" => Token::Keyword(Keyword::Let),
                    "if" => Token::Keyword(Keyword::If),
                    "else" => Token::Keyword(Keyword::Else),
                    "while" => Token::Keyword(Keyword::While),
                    _ => Token::Identifier(identifier),
                };
                result.push(token);
                column_counter += ident_len;
            }
            '"' => {
                let mut string_content = String::new();
                let mut str_total_len = 1;
                char_iter.next();
                while let Some(str_ch) = char_iter.next() {
                    match str_ch {
                        '\\' => {
                            let next_ch = match char_iter.next() {
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
                            break;
                        }
                        other_ch => {
                            string_content.push(other_ch);
                            str_total_len += 1;
                        }
                    }
                }
                result.push(Token::String(string_content));
                column_counter += str_total_len;
            }
            number if number.is_digit(10) => {
                let mut number_content = String::new();
                while let Some(digit) = char_iter.peek() {
                    if digit.is_digit(10) || *digit == '.' {
                        number_content.push(*digit);
                        char_iter.next();
                        column_counter += 1;
                    } else {
                        break;
                    }
                }
                let number_parsed = match number_content.parse::<f64>() {
                    Ok(n_p) => n_p,
                    Err(e) => {
                        return Err(format!(
                            "Error occurred when parsing number \"{}\": {}",
                            number_content, e
                        ));
                    }
                };
                result.push(Token::Number(number_parsed));
            }
            '{' => {
                char_iter.next();
                result.push(Token::Keyword(Keyword::LeftCurly));
                column_counter += 1;
            }
            '}' => {
                char_iter.next();
                result.push(Token::Keyword(Keyword::RightCurly));
                column_counter += 1;
            }
            '(' => {
                char_iter.next();
                result.push(Token::Keyword(Keyword::LeftParenthese));
                column_counter += 1;
            }
            ')' => {
                char_iter.next();
                result.push(Token::Keyword(Keyword::RightParenthese));
                column_counter += 1;
            }
            '=' => {
                char_iter.next();
                result.push(Token::Keyword(Keyword::Assign));
                column_counter += 1;
            }
            ',' => {
                char_iter.next();
                result.push(Token::Keyword(Keyword::Comma));
                column_counter += 1;
            }
            ' ' | '\t' => {
                char_iter.next();
                column_counter += 1;
            }
            '\r' => {
                //Sorry macOS users, no line counting for you
                char_iter.next();
            }
            '\n' => {
                char_iter.next();
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
