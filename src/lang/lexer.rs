pub fn get_tokens(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();

    for c in input.chars() {
        let token = Token::from_char(&c);

        if token.is_some() || c.is_whitespace() {
            if !buffer.is_empty() {
                if let Some(keyword) = Keyword::from_str(&buffer) {
                    tokens.push(Token::Keyword(keyword));
                } else {
                    tokens.push(Token::Identifier(buffer));
                }

                buffer = String::new();
            }
        }

        if let Some(token) = token {
            tokens.push(token);
        } else {
            if !c.is_whitespace() {
                buffer.push(c);
            }
        }
    }

    tokens
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    OpenBrace,
    CloseBrace,
    Colon,
    SemiColon,
    QuestionMark,
    Ampersand,
    AngleBracketOpen,
    AngleBracketClose,
    Comma,
    Equals,
}

impl Token {
    pub fn from_char(input: &char) -> Option<Self> {
        match input {
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            ':' => Token::Colon,
            ';' => Token::SemiColon,
            '?' => Token::QuestionMark,
            '&' => Token::Ampersand,
            '<' => Token::AngleBracketOpen,
            '>' => Token::AngleBracketClose,
            ',' => Token::Comma,
            '=' => Token::Equals,
            _ => return None,
        }
        .into()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Keyword {
    Alias,
    Struct,
    Enum,
    Extern,
}

impl Keyword {
    pub fn from_str(input: &str) -> Option<Self> {
        match input {
            "alias" => Keyword::Alias,
            "struct" => Keyword::Struct,
            "enum" => Keyword::Enum,
            "extern" => Keyword::Extern,
            _ => return None,
        }
        .into()
    }
}
