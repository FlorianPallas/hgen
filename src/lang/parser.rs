use super::{map::OrderedHashMap, schema::*};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "hgen.pest"]
pub struct SchemaParser;

pub fn parse_schema(source: &str) -> Schema {
    let mut pairs = SchemaParser::parse(Rule::hGEN, source).unwrap();
    let mut models = OrderedHashMap::new();
    let mut services = OrderedHashMap::new();

    while let Some(pair) = pairs.next() {
        match pair.as_rule() {
            Rule::Model => {
                let model_pair = pair.into_inner().next().unwrap();
                let (name, model) = parse_model(model_pair);
                models.insert(name, model);
            }
            Rule::Service => {
                let (name, service) = parse_service(pair);
                services.insert(name, service);
            }
            Rule::EOI => break,
            _ => panic!("unexpected top-level rule: {:?}", pair.as_rule()),
        }
    }

    Schema { models, services }
}

fn parse_model(pair: Pair<Rule>) -> (&str, Model) {
    match pair.as_rule() {
        Rule::Struct => {
            let (name, def) = parse_struct(pair);
            (name, Model::Struct(def))
        }
        Rule::Enum => {
            let (name, def) = parse_enum(pair);
            (name, Model::Enum(def))
        }
        Rule::Alias => {
            let (name, def) = parse_alias(pair);
            (name, Model::Alias(def))
        }
        Rule::External => {
            let (name, def) = parse_external(pair);
            (name, Model::External(def))
        }
        _ => panic!("unexpected model rule: {:?}", pair.as_rule()),
    }
}

fn parse_service(pair: Pair<Rule>) -> (&str, Service) {
    let mut pairs = pair.into_inner();

    let name = pairs.next().unwrap().as_str();

    let methods = pairs
        .map(parse_service_method)
        .collect::<OrderedHashMap<_, _>>();

    (name, Service { methods })
}

fn parse_service_method(pair: Pair<Rule>) -> (&str, Annotated<ServiceMethod>) {
    let mut pairs: pest::iterators::Pairs<Rule> = pair.into_inner();

    let name = pairs.next().unwrap().as_str();

    let inputs = pairs
        .next()
        .unwrap()
        .into_inner()
        .map(|pair| {
            pair.into_inner().map(|p| {
                let mut pairs = p.into_inner();

                let name = pairs.next().unwrap().as_str();
                let shape = parse_shape(pairs.next().unwrap());
                (name, shape)
            })
        })
        .flatten()
        .collect::<OrderedHashMap<_, _>>();

    let output = pairs.next().map(parse_shape);

    let metadata = pairs
        .next()
        .map(|pair| match parse_literal(pair) {
            Literal::Object(fields) => Some(fields),
            _ => panic!("unexpected metadata literal"),
        })
        .flatten()
        .unwrap_or_default();

    (
        name,
        Annotated {
            inner: ServiceMethod { inputs, output },
            metadata,
        },
    )
}

fn parse_struct(pair: Pair<Rule>) -> (&str, Struct) {
    let mut pairs = pair.into_inner();

    let name = pairs.next().unwrap().as_str();

    let fields = pairs
        .next()
        .unwrap()
        .into_inner()
        .map(|pair| {
            let mut pairs = pair.into_inner();
            let name = pairs.next().unwrap().as_str();
            let shape = parse_annotated_shape(pairs.next().unwrap());
            (name, shape)
        })
        .collect::<OrderedHashMap<_, _>>();

    (name, Struct { fields })
}

fn parse_enum(pair: Pair<Rule>) -> (&str, Enum) {
    let mut pairs = pair.into_inner();

    let name = pairs.next().unwrap().as_str();
    let fields = pairs.map(|pair| pair.as_str()).collect::<Vec<_>>();

    (name, Enum { fields })
}

fn parse_alias(pair: Pair<Rule>) -> (&str, Alias) {
    let mut pairs = pair.into_inner();

    let name = pairs.next().unwrap().as_str();
    let shape = parse_annotated_shape(pairs.next().unwrap());

    (name, Alias { shape })
}

fn parse_external(pair: Pair<Rule>) -> (&str, External) {
    let mut pairs = pair.into_inner();

    let name = pairs.next().unwrap().as_str();
    let shape = parse_annotated_shape(pairs.next().unwrap());

    (name, External { shape })
}

fn parse_shape(pair: Pair<Rule>) -> Shape {
    parse_annotated_shape(pair).inner
}

fn parse_annotated_shape(pair: Pair<Rule>) -> Annotated<Shape> {
    let mut pairs = pair.into_inner();

    let shape_pair = pairs.next().unwrap();
    let shape = match shape_pair.as_rule() {
        Rule::BoolShape => Shape::Bool,
        Rule::Int8Shape => Shape::Int8,
        Rule::Int16Shape => Shape::Int16,
        Rule::Int32Shape => Shape::Int32,
        Rule::Int64Shape => Shape::Int64,
        Rule::Float32Shape => Shape::Float32,
        Rule::Float64Shape => Shape::Float64,
        Rule::StringShape => Shape::String,
        Rule::ListShape => {
            let mut pairs = shape_pair.into_inner();
            let shape = parse_shape(pairs.next().unwrap());
            Shape::List(Box::new(shape))
        }
        Rule::MapShape => {
            let mut pairs = shape_pair.into_inner();
            let key_shape = parse_shape(pairs.next().unwrap());
            let value_shape = parse_shape(pairs.next().unwrap());
            Shape::Map(Box::new(key_shape), Box::new(value_shape))
        }
        Rule::ReferenceShape => Shape::Reference(shape_pair.as_str()),
        _ => panic!("unexpected shape rule: {:?}", shape_pair.as_rule()),
    };

    let mut is_nullable = false;
    let mut metadata = None;

    while let Some(pair) = pairs.next() {
        match pair.as_rule() {
            Rule::Nullable => {
                is_nullable = true;
            }
            Rule::ObjectLiteral => {
                metadata = match parse_literal(pair) {
                    Literal::Object(fields) => Some(fields),
                    _ => panic!("unexpected metadata literal"),
                };
            }
            _ => panic!("unexpected shape rule: {:?}", pair.as_rule()),
        }
    }

    Annotated {
        inner: if is_nullable {
            Shape::Nullable(Box::new(shape))
        } else {
            shape
        },
        metadata: metadata.unwrap_or_default(),
    }
}

fn parse_literal(pair: Pair<Rule>) -> Literal {
    match pair.as_rule() {
        Rule::BoolLiteral => Literal::Bool(pair.as_str().parse().unwrap()),
        Rule::IntLiteral => Literal::Int(pair.as_str().parse().unwrap()),
        Rule::FloatLiteral => Literal::Float(pair.as_str().parse().unwrap()),
        Rule::StringLiteral => Literal::String(&pair.as_str()[1..pair.as_str().len() - 1]),
        Rule::ObjectLiteral => {
            let mut pairs = pair.into_inner();
            let mut fields = OrderedHashMap::new();

            while let Some(pair) = pairs.next() {
                let value = parse_literal(pairs.next().unwrap());
                fields.insert(pair.as_str(), value);
            }

            Literal::Object(fields)
        }
        Rule::ArrayLiteral => {
            let values = pair.into_inner().map(parse_literal).collect();
            Literal::Array(values)
        }
        _ => panic!("unexpected literal rule: {:?}", pair.as_rule()),
    }
}
