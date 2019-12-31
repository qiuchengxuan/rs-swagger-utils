use std::collections::HashMap;
use std::sync::atomic::{AtomicPtr, Ordering};

use yaml_rust::Yaml;

use super::format::{FormatValidator, NO_FORMAT, UNKNOWN_FORMAT};
use super::Validator;
use common::IntegerType;

type Formats = HashMap<&'static str, &'static dyn FormatValidator<i64>>;

lazy_static! {
    pub static ref FORMATS: AtomicPtr<Formats> =
        AtomicPtr::new(&mut HashMap::new() as *mut Formats);
}

pub fn set_formats(formats: &Formats) {
    let mut new_formats: Formats = formats.clone();
    FORMATS.store(&mut new_formats, Ordering::Relaxed)
}

#[derive(Clone)]
pub struct IntegerValidator {
    format: &'static dyn FormatValidator<i64>,
    pub minimum: i64,
    pub maximum: i64,
}

impl<'a> From<&IntegerType> for IntegerValidator {
    fn from(type_integer: &IntegerType) -> Self {
        let formats = unsafe { &*FORMATS.load(Ordering::Relaxed) };
        let format = match &type_integer.format {
            Some(name) => match formats.get(name.as_str()) {
                Some(&v) => v,
                None => &UNKNOWN_FORMAT,
            },
            None => &NO_FORMAT,
        };
        IntegerValidator {
            format,
            minimum: type_integer.minimum.unwrap_or(std::i64::MIN),
            maximum: type_integer.maximum.unwrap_or(std::i64::MAX),
        }
    }
}

impl Validator<i64> for IntegerValidator {
    fn validate(&self, value: &i64) -> Option<String> {
        if *value < self.minimum {
            return some_str!("field is too small");
        }
        if *value > self.maximum {
            return some_str!("field is too large");
        }
        if !self.format.validate(*value) {
            return some_str!("field is not format of {}", self.format);
        }
        None
    }
}

impl Validator<Yaml> for IntegerValidator {
    fn validate(&self, yaml: &Yaml) -> Option<String> {
        match yaml {
            Yaml::Integer(i) => self.validate(i),
            _ => return some_str!("field is not integer"),
        }
    }
}

impl Default for IntegerValidator {
    fn default() -> IntegerValidator {
        IntegerValidator {
            format: &NO_FORMAT,
            minimum: std::i64::MIN,
            maximum: std::i64::MAX,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate yaml_rust;
    use super::IntegerValidator;
    use validator::Validator;
    use yaml_rust::YamlLoader;

    #[test]
    fn test_integer() {
        let s = "
        a: 1
        b: 10
        c: c
        ";
        let docs = YamlLoader::load_from_str(s).unwrap();
        let doc = &docs[0];
        let v = IntegerValidator::default();
        assert_eq!(v.validate(&doc["a"]), None);
        let v = IntegerValidator {
            minimum: 2,
            ..Default::default()
        };
        assert_eq!(v.validate(&doc["a"]), some_str!("field is too small"));
        let v = IntegerValidator {
            maximum: 2,
            ..Default::default()
        };
        assert_eq!(v.validate(&doc["b"]), some_str!("field is too large"));
        assert_eq!(v.validate(&doc["c"]), some_str!("field is not integer"));
    }
}
