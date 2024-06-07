#[derive(Debug)]
struct Part {
    value: char,
    token: Option<Token>,
}

impl Part {
    fn new(value: char) -> Self {
        Self {
            value,
            token: Token::from_char(&value),
        }
    }
}

pub fn get_tokens(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut parts = input.chars().map(Part::new).peekable();

    loop {
        let Part { value, token } = match parts.next() {
            Some(c) => c,
            None => break,
        };

        // handle string literals
        if token == Some(Token::Quote) {
            let mut buffer = String::new();

            // consume characters until we find the closing quote
            while let Some(Part { value, token }) = parts.next() {
                if token == Some(Token::Quote) {
                    tokens.push(Token::StringLiteral(buffer));
                    break;
                }
                buffer.push(value);
            }

            continue;
        }

        if token.is_some() || value.is_whitespace() {
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
            if !value.is_whitespace() {
                buffer.push(value);
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
    Quote,
    StringLiteral(String),
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
            '"' => Token::Quote,
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
