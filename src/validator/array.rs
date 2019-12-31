use yaml_rust::Yaml;

use super::common::to_validator;
use super::reference::ValidatorQuerier;
use super::Validator;
use common::Attribute;

pub struct ArrayValidator<'a> {
    validator: Box<dyn Validator<Yaml> + 'a>,
}

impl<'a, 'b> ArrayValidator<'a> {
    pub fn new(attr: &Attribute, querier: &'a dyn ValidatorQuerier) -> Self {
        ArrayValidator {
            validator: to_validator(attr, querier),
        }
    }
}

impl<'a> Validator<Yaml> for ArrayValidator<'a> {
    fn validate(&self, yaml: &Yaml) -> Option<String> {
        match yaml {
            Yaml::Array(array) => {
                for entry in array.iter() {
                    let option = self.validator.validate(entry);
                    if option.is_some() {
                        return option;
                    }
                }
                None
            }
            _ => some_str!("field is not array"),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate yaml_rust;

    use yaml_rust::YamlLoader;

    use super::{ArrayValidator, Validator};
    use common::Attribute;
    use validator::object::ObjectValidator;
    use validator::reference::{Location, ValidatorQuerier};

    pub(crate) struct NoneQuerier;

    impl ValidatorQuerier for NoneQuerier {
        fn get(&self, _: &Location) -> Option<ObjectValidator> {
            None
        }
    }

    #[test]
    fn test_array() {
        let schema: Attribute = serde_yaml::from_str("type: integer").unwrap();

        let querier = NoneQuerier {};

        let v = ArrayValidator::new(&schema, &querier);

        let docs = YamlLoader::load_from_str("a: [1, 2]").unwrap();
        let doc = &docs[0];
        assert_eq!(v.validate(&doc["a"]), None);

        let docs = YamlLoader::load_from_str("a: [1, b]").unwrap();
        let doc = &docs[0];
        assert_eq!(v.validate(&doc["a"]), some_str!("field is not integer"));

        let docs = YamlLoader::load_from_str("a: 1").unwrap();
        let doc = &docs[0];
        assert_eq!(v.validate(&doc["a"]), some_str!("field is not array"));
    }
}
