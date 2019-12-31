use std::iter::Iterator;

use super::{In, Operation, Parameter};
use common::TypeDefinition;
use validator::integer::IntegerValidator;
use validator::string::StringValidator;

pub enum Segment<'a> {
    Fixed(&'a str),
    Text(StringValidator),
    Number(IntegerValidator),
}

pub struct SegmentIter<'a> {
    tokens: Vec<&'a str>,
    parameters: &'a Option<Vec<Parameter>>,
    token_index: usize,
}

impl<'a> Iterator for SegmentIter<'a> {
    type Item = Segment<'a>;

    fn next(&mut self) -> Option<Segment<'a>> {
        if self.token_index >= self.tokens.len() {
            return None;
        }
        let token = self.tokens[self.token_index];
        self.token_index += 1;
        if !token.starts_with("{") || !token.ends_with("}") {
            return Some(Segment::Fixed(token));
        }

        let token = &token[1..token.len() - 1];
        let parameters = match self.parameters {
            Some(parameters) => parameters,
            None => return Some(Segment::Fixed(token)),
        };

        for parameter in parameters.iter() {
            if parameter.name != token || parameter.in_ != In::Path {
                continue;
            }
            return match parameter.attribute.definition.as_ref().unwrap() {
                TypeDefinition::Integer(integer_type) => {
                    Some(Segment::Number(IntegerValidator::from(integer_type)))
                }
                TypeDefinition::String(string_type) => {
                    Some(Segment::Text(StringValidator::from(string_type)))
                }
                _ => Some(Segment::Text(StringValidator::default())),
            };
        }
        return None;
    }
}

impl<'a> From<(&'a String, &'a Operation)> for SegmentIter<'a> {
    fn from(tuple: (&'a String, &'a Operation)) -> Self {
        let (path, operation) = tuple;
        let tokens: Vec<&str> = path[1..].split("/").collect();
        SegmentIter {
            tokens,
            parameters: &operation.parameters,
            token_index: 0,
        }
    }
}
