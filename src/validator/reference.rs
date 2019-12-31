use std::collections::HashMap;

use yaml_rust::Yaml;

use super::object::ObjectValidator;
use super::Validator;
use common::{Attribute, TypeDefinition};

#[derive(Clone, PartialEq, Debug)]
pub enum Location {
    Local(String),
    Unknown,
}

impl From<String> for Location {
    fn from(uri: String) -> Location {
        if !uri.contains('#') {
            return Location::Unknown;
        }
        let splitted: Vec<&str> = uri.splitn(2, '#').collect();
        if splitted.len() != 2 || splitted[0].len() > 0 {
            return Location::Unknown;
        }
        if !splitted[1].contains("/definitions/") {
            return Location::Unknown;
        }
        Location::Local(splitted[1].replace("/definitions/", ""))
    }
}

pub trait ValidatorQuerier {
    fn get(&self, location: &Location) -> Option<ObjectValidator>;
}

impl<'a> ValidatorQuerier for HashMap<String, Attribute> {
    fn get(&self, location: &Location) -> Option<ObjectValidator> {
        match location {
            Location::Local(path) => {
                if !self.contains_key(path) {
                    return None;
                }
                let attr = self.get(path).unwrap();
                if !attr.definition.is_some() {
                    return None;
                }
                match attr.definition.as_ref().unwrap() {
                    TypeDefinition::Object(object) => {
                        let querier: &dyn ValidatorQuerier = self;
                        Some(ObjectValidator::new(&object, querier))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

pub(crate) struct ReferenceValidator<'a> {
    pub(crate) location: Location,
    pub(crate) querier: &'a dyn ValidatorQuerier,
}

impl<'a> Validator<Yaml> for ReferenceValidator<'a> {
    fn validate(&self, yaml: &Yaml) -> Option<String> {
        match self.querier.get(&self.location) {
            Some(v) => v.validate(yaml),
            None => some_str!("No such reference"),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate yaml_rust;

    use std::collections::HashMap;

    use yaml_rust::YamlLoader;

    use super::{Location, ReferenceValidator, Validator};
    use common::Attribute;

    #[test]
    fn test_reference() {
        let schema = "
        Test:
          type: object
          required:
            - id
            - name
          properties:
            id:
              type: integer
            name:
              type: string
            valid:
              type: boolean
        ";
        let attributes: HashMap<String, Attribute> = serde_yaml::from_str(&schema).unwrap();

        let v = ReferenceValidator {
            location: Location::from("#/definitions/Test".to_string()),
            querier: &attributes,
        };

        let s = "
        id: 1
        name: test
        valid: true
        ";
        let docs = YamlLoader::load_from_str(s).unwrap();
        let doc = &docs[0];
        assert_eq!(v.validate(&doc), None);
    }
}
