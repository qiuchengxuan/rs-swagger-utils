use yaml_rust::Yaml;

use super::array::ArrayValidator;
use super::boolean::BooleanValidator;
use super::integer::IntegerValidator;
use super::object::ObjectValidator;
use super::reference::{Location, ReferenceValidator, ValidatorQuerier};
use super::string::StringValidator;
use super::{UnknownValidator, Validator};
use common::{Attribute, TypeDefinition};

pub(crate) fn to_validator<'a>(
    attribute: &Attribute, querier: &'a dyn ValidatorQuerier,
) -> Box<dyn Validator<Yaml> + 'a> {
    if attribute.reference.is_none() {
        match attribute.definition.as_ref().unwrap() {
            TypeDefinition::Boolean => Box::new(BooleanValidator {}),
            TypeDefinition::Integer(integer_type) => Box::new(IntegerValidator::from(integer_type)),
            TypeDefinition::String(string_type) => Box::new(StringValidator::from(string_type)),
            TypeDefinition::Array { items } => Box::new(ArrayValidator::new(items, querier)),
            TypeDefinition::Object(object_type) => {
                Box::new(ObjectValidator::new(object_type, querier))
            }
            _ => Box::new(UnknownValidator {}),
        }
    } else {
        let ref_name = attribute.reference.as_ref().unwrap();
        Box::new(ReferenceValidator {
            location: Location::from(ref_name.to_string()),
            querier,
        })
    }
}
