use console::style;
use std::{collections::HashMap, iter::Peekable, vec::IntoIter};
use thiserror::Error;

use super::lexer::{Keyword, Token};
use super::schema::*;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected end of input at token")]
    NoToken,
    #[error("Unexpected token, expected {expected:?}")]
    UnexpectedToken { expected: Token },
}

pub fn get_schema(tokens: Vec<Token>) -> Result<Schema, ParseError> {
    let mut context = Context::new(tokens.clone());

    let schema = parse_schema(&mut context);
    if let Err(_err) = &schema {
        print_parse_debug(&tokens, context.index - 1);
    }

    schema
}

fn print_parse_debug(tokens: &Vec<Token>, index: usize) {
    let out = tokens
        .iter()
        .enumerate()
        .map(|(i, token)| {
            format!(
                "{:?}",
                if i == index {
                    style(token).red()
                } else {
                    style(token).dim()
                }
            )
        })
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", out);
}

struct Context {
    iter: Peekable<IntoIter<Token>>,
    index: usize,
}

impl Context {
    pub fn new(tokens: Vec<Token>) -> Self {
        Context {
            iter: tokens.into_iter().peekable(),
            index: 0,
        }
    }
}

impl Context {
    pub fn peek(&mut self) -> Option<&Token> {
        self.iter.peek()
    }

    pub fn pop(&mut self) -> Option<Token> {
        let token = self.iter.next()?;
        self.index += 1;
        Some(token)
    }

    pub fn pop_if(&mut self, token: Token) -> Option<Token> {
        if self.peek().filter(|t| **t == token).is_some() {
            self.pop()
        } else {
            None
        }
    }

    pub fn pop_strict(&mut self) -> Result<Token, ParseError> {
        self.pop().ok_or(ParseError::NoToken)
    }

    pub fn pop_exact(&mut self, token: Token) -> Result<Token, ParseError> {
        self.pop_strict().map(|actual| {
            if actual == token {
                Ok(actual)
            } else {
                Err(ParseError::UnexpectedToken { expected: token })
            }
        })?
    }

    pub fn pop_identifier(&mut self) -> Result<String, ParseError> {
        self.pop_strict().map(|token| {
            match token {
                Token::Identifier(value) => Some(value),
                _ => None,
            }
            .ok_or(ParseError::UnexpectedToken {
                expected: Token::Identifier("".to_owned()),
            })
        })?
    }
}

fn parse_schema(context: &mut Context) -> Result<Schema, ParseError> {
    let mut models = Vec::new();
    let mut services = Vec::new();

    loop {
        if let Some(token) = context.peek() {
            match token {
                Token::Keyword(Keyword::Struct) => {
                    models.push(Model::Struct(parse_struct(context)?));
                }
                Token::Keyword(Keyword::Enum) => {
                    models.push(Model::Enum(parse_enum(context)?));
                }
                Token::Keyword(Keyword::Alias) => {
                    models.push(Model::Alias(parse_alias(context)?));
                }
                Token::Keyword(Keyword::Extern) => {
                    models.push(Model::External(parse_extern_type(context)?));
                }
                Token::Keyword(Keyword::Service) => {
                    services.push(parse_service(context)?);
                }
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: token.clone(),
                    })
                }
            }
        } else {
            break;
        }
    }

    Ok(Schema { models, services })
}

fn parse_service(context: &mut Context) -> Result<Service, ParseError> {
    context.pop_exact(Token::Keyword(Keyword::Service))?;
    let name = context.pop_identifier()?;
    context.pop_exact(Token::OpenBrace)?;

    let mut methods = Vec::new();

    loop {
        if context.pop_if(Token::CloseBrace).is_some() {
            break Ok(Service { name, methods });
        }

        let name = context.pop_identifier()?;
        context.pop_exact(Token::OpenParen)?;

        let mut request = Vec::new();
        loop {
            if context.pop_if(Token::CloseParen).is_some() {
                break;
            }

            let name = context.pop_identifier()?;
            context.pop_exact(Token::Colon)?;
            let def = parse_type(context)?;
            request.push((name, def));
            context.pop_if(Token::Comma);
        }

        context.pop_exact(Token::Dash)?;
        context.pop_exact(Token::AngleBracketClose)?;

        let response = parse_type(context)?;
        context.pop_exact(Token::Comma)?;
        methods.push(Method {
            name,
            inputs: request,
            output: response,
        });
    }
}

fn parse_extern_type(context: &mut Context) -> Result<External, ParseError> {
    context.pop_exact(Token::Keyword(Keyword::Extern))?;
    context.pop_exact(Token::Keyword(Keyword::Alias))?;
    let name = context.pop_identifier()?;
    context.pop_exact(Token::Equals)?;
    let def = parse_type(context)?;
    context.pop_exact(Token::SemiColon)?;

    Ok(External { name, def })
}

fn parse_alias(context: &mut Context) -> Result<Alias, ParseError> {
    context.pop_exact(Token::Keyword(Keyword::Alias))?;
    let name = context.pop_identifier()?;
    context.pop_exact(Token::Equals)?;
    let def = parse_type(context)?;
    context.pop_exact(Token::SemiColon)?;

    Ok(Alias { name, def })
}

fn parse_struct(context: &mut Context) -> Result<Struct, ParseError> {
    context.pop_exact(Token::Keyword(Keyword::Struct))?;
    let name = context.pop_identifier()?;
    context.pop_exact(Token::OpenBrace)?;

    let mut fields = Vec::new();

    loop {
        if context.pop_if(Token::CloseBrace).is_some() {
            break Ok(Struct { name, fields });
        }

        let name = context.pop_identifier()?;
        context.pop_exact(Token::Colon)?;
        let def = parse_type(context)?;
        context.pop_exact(Token::Comma)?;
        fields.push((name, def));
    }
}

fn parse_enum(context: &mut Context) -> Result<Enum, ParseError> {
    context.pop_exact(Token::Keyword(Keyword::Enum))?;
    let name = context.pop_identifier()?;
    context.pop_exact(Token::OpenBrace)?;

    let mut fields = Vec::new();

    loop {
        if context.pop_if(Token::CloseBrace).is_some() {
            break Ok(Enum {
                name,
                values: fields,
            });
        }

        let name = context.pop_identifier()?;
        context.pop_exact(Token::Comma)?;

        fields.push(name);
    }
}

fn parse_type(context: &mut Context) -> Result<Type, ParseError> {
    let name = context.pop_identifier()?;
    let mut args = Vec::new();

    if context.pop_if(Token::AngleBracketOpen).is_some() {
        loop {
            args.push(context.pop_identifier()?);

            if context.pop_if(Token::AngleBracketClose).is_some() {
                break;
            }

            context.pop_exact(Token::Comma)?;
        }
    }

    let mut data = HashMap::new();
    if context.pop_if(Token::Ampersand).is_some() {
        context.pop_exact(Token::OpenBrace)?;

        loop {
            let key = context.pop_identifier()?;
            context.pop_exact(Token::Colon)?;
            let value = context.pop_identifier()?;
            context.pop_exact(Token::Comma)?;

            data.insert(key, value);

            if context.pop_if(Token::CloseBrace).is_some() {
                break;
            }
        }
    }

    let mut shape = parse_shape(name, args);
    if context.pop_if(Token::QuestionMark).is_some() {
        shape = Shape::Nullable(Box::new(shape));
    }

    Ok(Type { shape, data })
}

fn parse_shape(name: String, args: Vec<String>) -> Shape {
    if let Ok(primitive) = Primitive::try_from(name.as_str()) {
        return Shape::Primitive(primitive);
    }

    match name.as_str() {
        "Optional" => {
            let [inner] = &args[..] else {
                panic!("Expected one argument for Optional but got {:?}", args)
            };
            Shape::Nullable(Box::new(parse_shape(inner.to_owned(), vec![])))
        }
        "List" => {
            let [inner] = &args[..] else {
                panic!("Expected one argument for List but got {:?}", args)
            };
            Shape::List(Box::new(parse_shape(inner.to_owned(), vec![])))
        }
        "Set" => {
            let [inner] = &args[..] else {
                panic!("Expected one argument for Set but got {:?}", args)
            };
            Shape::Set(Box::new(parse_shape(inner.to_owned(), vec![])))
        }
        "Map" => {
            let [key, value] = &args[..] else {
                panic!("Expected two arguments for Map but got {:?}", args)
            };
            Shape::Map(
                Box::new(parse_shape(key.to_owned(), vec![])),
                Box::new(parse_shape(value.to_owned(), vec![])),
            )
        }
        _ => Shape::Reference(name),
    }
}
