pub enum Keyword {
    Fn,
    Let,
}

pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Number(f64),
}

pub fn tokenize(code: &str) -> Vec<Token> {
    let mut counter: u32 = 0;
    let mut result = Vec::<Token>::new();
    while counter < code.len() {
        match code[counter] {
            letter if letter.is_alphabetic() || letter == '_' => {
                //Either keyword or identifier
                let mut identifier = String::new();
                let mut ident_counter = counter + 1;
                while ident_counter < code.len() {
                    let ident_letter = code[ident_counter];
                    if ident_letter.is_alphanumeric() || ident_letter == '_' {
                        identifier.push(ident_letter);
                    } else {
                        break;
                    }
                }

                let token = match identifier {
                    "fn" => Token::Keyword(Keyword::Fn),
                    "let" => Token::Keyword(Keyword::Let),
                    _ => Token::Identifier(identifier),
                };

                counter = ident_counter - 1;
            }
        }
        counter += 1;
    }
    result
}
