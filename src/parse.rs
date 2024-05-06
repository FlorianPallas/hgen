use std::{collections::HashMap, iter::Peekable, vec::IntoIter};

use crate::{EnumDef, FieldDef, ModuleDef, ObjectDef, SimpleTypeDef, TypeDef};

pub fn parse(input: &str) -> ModuleDef {
    let tokens = get_tokens(input);
    let mut context = Context {
        iter: tokens.into_iter().peekable(),
    };

    read_module(&mut context)
}

struct Context {
    iter: Peekable<IntoIter<Token>>,
}

impl Context {
    pub fn pop(&mut self, expected: Token) -> Option<Token> {
        let actual = self.iter.next().unwrap();
        if actual == expected {
            Some(actual)
        } else {
            None
        }
    }

    pub fn pop_if(&mut self, expected: Token) -> Option<Token> {
        if self.iter.peek() == Some(&expected) {
            self.iter.next()
        } else {
            None
        }
    }

    pub fn read_token(&mut self, token: Token) -> Token {
        self.pop(token).unwrap()
    }

    pub fn read_identifier(&mut self) -> String {
        if let Some(token) = self.iter.next() {
            match token {
                Token::Identifier(s) => s,
                _ => panic!("Expected identifier but got {:?}", token),
            }
        } else {
            panic!("Expected identifier but got None");
        }
    }
}

fn read_module(context: &mut Context) -> ModuleDef {
    let mut objects = Vec::new();
    let mut enums = Vec::new();

    loop {
        if let Some(token) = context.iter.peek() {
            match token {
                Token::Keyword(Keyword::Interface) => {
                    objects.push(read_object(context));
                }
                Token::Keyword(Keyword::Enum) => {
                    enums.push(read_enum(context));
                }
                _ => panic!("Expected interface or enum but got {:?}", token),
            }
        } else {
            break;
        }
    }

    ModuleDef {
        name: "api".to_owned(),
        objects,
        enums,
    }
}

fn read_object(context: &mut Context) -> ObjectDef {
    context.read_token(Token::Keyword(Keyword::Interface));
    let name = context.read_identifier();
    context.read_token(Token::OpenBrace);

    let mut fields = Vec::new();

    loop {
        if context.pop_if(Token::CloseBrace).is_some() {
            break ObjectDef { name, fields };
        }

        fields.push(read_property(context));
    }
}

fn read_enum(context: &mut Context) -> EnumDef {
    context.read_token(Token::Keyword(Keyword::Enum));
    let name = context.read_identifier();
    context.read_token(Token::OpenBrace);

    let mut values = Vec::new();

    loop {
        if context.pop_if(Token::CloseBrace).is_some() {
            break;
        }

        values.push(context.read_identifier());
        context.read_token(Token::Comma);
    }

    EnumDef { name, values }
}

fn read_property(context: &mut Context) -> FieldDef {
    let name = context.read_identifier();
    let nullable = context.pop_if(Token::QuestionMark).is_some();

    context.read_token(Token::Colon);

    let identifier = context.read_identifier();
    let mut args = Vec::new();
    if context.pop_if(Token::AngleBracketOpen).is_some() {
        loop {
            args.push(context.read_identifier());

            if context.pop_if(Token::AngleBracketClose).is_some() {
                break;
            }

            context.read_token(Token::Comma);
        }
    }

    let mut metadata = HashMap::new();
    if context.pop_if(Token::Ampersand).is_some() {
        context.pop(Token::OpenBrace);

        loop {
            let key = context.read_identifier();
            context.read_token(Token::Colon);
            let value = context.read_identifier();
            context.read_token(Token::SemiColon);

            metadata.insert(key, value);

            if context.pop_if(Token::CloseBrace).is_some() {
                break;
            }
        }
    }

    context.read_token(Token::SemiColon);

    let type_def = if args.is_empty() {
        TypeDef::Simple(SimpleTypeDef::from_str(&identifier))
    } else {
        match identifier.as_str() {
            "List" => {
                let [inner] = &args[..] else {
                    panic!("Expected one argument for List but got {:?}", args);
                };

                TypeDef::List(Box::new(SimpleTypeDef::from_str(inner)))
            }
            "Set" => {
                let [inner] = &args[..] else {
                    panic!("Expected one argument for Set but got {:?}", args);
                };

                TypeDef::Set(Box::new(SimpleTypeDef::from_str(inner)))
            }
            "Map" => {
                let [key, value] = &args[..] else {
                    panic!("Expected two arguments for Map but got {:?}", args);
                };

                TypeDef::Map(
                    Box::new(SimpleTypeDef::from_str(key)),
                    Box::new(SimpleTypeDef::from_str(value)),
                )
            }
            _ => panic!("Unknown type {}", identifier),
        }
    };

    FieldDef {
        name,
        metadata,
        nullable,
        type_def,
    }
}

fn get_tokens(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();

    for c in input.chars() {
        let token = match c {
            '{' => Some(Token::OpenBrace),
            '}' => Some(Token::CloseBrace),
            ':' => Some(Token::Colon),
            ';' => Some(Token::SemiColon),
            '?' => Some(Token::QuestionMark),
            '&' => Some(Token::Ampersand),
            '<' => Some(Token::AngleBracketOpen),
            '>' => Some(Token::AngleBracketClose),
            ',' => Some(Token::Comma),
            _ => None,
        };

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
enum Token {
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
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Keyword {
    Interface,
    Enum,
}

impl Keyword {
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "interface" => Some(Keyword::Interface),
            "enum" => Some(Keyword::Enum),
            _ => None,
        }
    }
}
