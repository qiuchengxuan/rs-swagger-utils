use std::collections::HashMap;

use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

use super::common::to_validator;
use super::reference::ValidatorQuerier;
use super::Validator;
use common::ObjectType;

pub struct ObjectValidator<'a> {
    properties: HashMap<String, Box<dyn Validator<Yaml> + 'a>>,
    required: Vec<String>,
}

impl<'a, 'b> ObjectValidator<'a> {
    pub fn new(obj_type: &ObjectType, querier: &'a dyn ValidatorQuerier) -> Self {
        let cap = obj_type.properties.len();
        let mut properties: HashMap<String, Box<dyn Validator<Yaml> + 'a>> =
            HashMap::with_capacity(cap);
        for (k, v) in obj_type.properties.iter() {
            properties.insert(k.into(), to_validator(v, querier));
        }

        ObjectValidator {
            properties,
            required: match &obj_type.required {
                Some(r) => r.clone(),
                None => Vec::new(),
            },
        }
    }

    fn validate_attributes(&self, hash: &Hash) -> Option<String> {
        for name in self.required.iter() {
            if !hash.contains_key(&Yaml::String(name.into())) {
                return some_str!("Field {} is required", name);
            }
        }
        for (field, value) in hash.iter() {
            match field {
                Yaml::String(s) => {
                    let option = self.properties.get(s);
                    if option.is_none() {
                        return some_str!("Unknown field {}", s);
                    }
                    let result = option.unwrap().validate(value);
                    if result.is_some() {
                        return result;
                    }
                }
                _ => return some_str!("Unexpected field type"),
            }
        }
        None
    }
}

impl<'a> Validator<Yaml> for ObjectValidator<'a> {
    fn validate(&self, yaml: &Yaml) -> Option<String> {
        match yaml {
            Yaml::Hash(hash) => self.validate_attributes(hash),
            _ => some_str!("field is not object"),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate yaml_rust;

    use yaml_rust::YamlLoader;

    use super::{ObjectValidator, Validator};
    use common::{Attribute, TypeDefinition};
    use validator::reference::{Location, ValidatorQuerier};

    pub(crate) struct NoneQuerier;

    impl ValidatorQuerier for NoneQuerier {
        fn get(&self, _: &Location) -> Option<ObjectValidator> {
            None
        }
    }

    #[test]
    fn test_basic_object() {
        let schema = "
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
        let test_schema: Attribute = serde_yaml::from_str(&schema).unwrap();
        let option = match test_schema.definition.unwrap() {
            TypeDefinition::Object(object_type) => Some(object_type),
            _ => None,
        };
        assert!(option.is_some());
        let object_type = option.unwrap();

        let querier = NoneQuerier {};
        let v = ObjectValidator::new(&object_type, &querier);

        let docs = YamlLoader::load_from_str("id: 1").unwrap();
        let doc = &docs[0];
        assert_eq!(v.validate(&doc), some_str!("Field name is required"));

        let s = "
        id: 1
        name: test
        whatever: true
        ";
        let docs = YamlLoader::load_from_str(s).unwrap();
        let doc = &docs[0];
        assert_eq!(v.validate(&doc), some_str!("Unknown field whatever"));

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
