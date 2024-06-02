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
    OpenParen,
    CloseParen,
    Colon,
    SemiColon,
    QuestionMark,
    Ampersand,
    AngleBracketOpen,
    AngleBracketClose,
    Comma,
    Equals,
    Dash,
}

impl Token {
    pub fn from_char(input: &char) -> Option<Self> {
        match input {
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            ':' => Token::Colon,
            ';' => Token::SemiColon,
            '?' => Token::QuestionMark,
            '&' => Token::Ampersand,
            '<' => Token::AngleBracketOpen,
            '>' => Token::AngleBracketClose,
            ',' => Token::Comma,
            '=' => Token::Equals,
            '-' => Token::Dash,
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
    Service,
    Use,
}

impl Keyword {
    pub fn from_str(input: &str) -> Option<Self> {
        match input {
            "alias" => Keyword::Alias,
            "struct" => Keyword::Struct,
            "enum" => Keyword::Enum,
            "extern" => Keyword::Extern,
            "service" => Keyword::Service,
            "use" => Keyword::Use,
            _ => return None,
        }
        .into()
    }
}
