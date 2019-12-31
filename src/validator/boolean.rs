use super::Validator;

use yaml_rust::Yaml;

pub struct BooleanValidator;

impl Validator<Yaml> for BooleanValidator {
    fn validate(&self, yaml: &Yaml) -> Option<String> {
        match yaml {
            Yaml::Boolean(_) => None,
            _ => return some_str!("field is not boolean"),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate yaml_rust;
    use validator::boolean::BooleanValidator;
    use validator::Validator;
    use yaml_rust::YamlLoader;

    #[test]
    fn test_boolean() {
        let s = "
        a: true
        b: 1
        c: test
        ";
        let docs = YamlLoader::load_from_str(s).unwrap();
        let doc = &docs[0];
        let v = BooleanValidator {};
        assert_eq!(v.validate(&doc["a"]), None);
        assert_eq!(v.validate(&doc["b"]), some_str!("field is not boolean"));
        assert_eq!(v.validate(&doc["c"]), some_str!("field is not boolean"));
    }
}
