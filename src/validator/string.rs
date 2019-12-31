use std::collections::HashMap;
use std::collections::HashSet;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::atomic::{AtomicPtr, Ordering};

use derive_more::Display;
use yaml_rust::Yaml;

use super::format::{FormatValidator, NO_FORMAT, UNKNOWN_FORMAT};
use super::Validator;
use common::StringType;

#[derive(Clone, Display)]
#[display(fmt = "IPv4")]
struct IPv4Format;

impl FormatValidator<String> for IPv4Format {
    fn validate(&self, s: String) -> bool {
        Ipv4Addr::from_str(&s).is_ok()
    }
}

const IPV4_FORMAT: &dyn FormatValidator<String> = &IPv4Format {};

pub const STRING_FORMATS: [(&str, &dyn FormatValidator<String>); 1] = [("ipv4", IPV4_FORMAT)];

type Formats = HashMap<&'static str, &'static dyn FormatValidator<String>>;

lazy_static! {
    pub static ref FORMATS: AtomicPtr<Formats> =
        AtomicPtr::new(&mut STRING_FORMATS.iter().cloned().collect() as *mut Formats);
}

pub fn set_formats(formats: &Formats) {
    let mut new_formats: Formats = formats.clone();
    FORMATS.store(&mut new_formats, Ordering::Relaxed)
}

#[derive(Clone)]
pub struct StringValidator {
    pub format: &'static dyn FormatValidator<String>,
    pub choices: HashSet<String>,
}

impl<'a> From<&'a StringType> for StringValidator {
    fn from(type_string: &StringType) -> StringValidator {
        let formats = unsafe { &*FORMATS.load(Ordering::Relaxed) };
        let format = match &type_string.format.as_ref() {
            Some(name) => match formats.get(name.as_str()) {
                Some(&v) => v,
                None => &UNKNOWN_FORMAT,
            },
            None => &NO_FORMAT,
        };
        StringValidator {
            format,
            choices: match &type_string.choices {
                Some(choices) => choices.clone(),
                None => HashSet::default(),
            },
        }
    }
}

impl StringValidator {
    fn format_choices(&self) -> String {
        let mut s = String::new();
        s.push_str("[");
        for choice in self.choices.iter() {
            s.push_str(choice);
            s.push_str(", ")
        }
        s.truncate(s.len() - 2);
        s.push_str("]");
        s.into()
    }
}

impl Validator<str> for StringValidator {
    fn validate(&self, s: &str) -> Option<String> {
        if !self.format.validate(s.to_string()) {
            return some_str!("field is not format of {}", self.format);
        }
        if self.choices.len() > 0 && !self.choices.contains(s) {
            return some_str!("field is not one of {}", self.format_choices());
        }
        None
    }
}

impl Validator<Yaml> for StringValidator {
    fn validate(&self, yaml: &Yaml) -> Option<String> {
        match yaml {
            Yaml::String(s) => self.validate(&s as &str),
            _ => return some_str!("field is not string"),
        }
    }
}

impl Default for StringValidator {
    fn default() -> StringValidator {
        StringValidator {
            format: &NO_FORMAT,
            choices: HashSet::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate yaml_rust;

    use std::collections::HashSet;
    use yaml_rust::YamlLoader;

    use super::{set_formats, Formats, StringValidator, IPV4_FORMAT, STRING_FORMATS};
    use validator::format::NO_FORMAT;
    use validator::Validator;

    #[test]
    fn test_ipv4_format() {
        assert_eq!(IPV4_FORMAT.validate("1.1.1.1".to_string()), true);
    }

    #[test]
    fn test_string_validator() {
        let formats: Formats = STRING_FORMATS.iter().cloned().collect();
        set_formats(&formats);
        // let formats: Formats = STRING_FORMATS.iter().cloned().collect();
        let s = "
        a: CAT
        b: 2
        c: 1.1.1.1
        ";
        let docs = YamlLoader::load_from_str(s).unwrap();
        let doc = &docs[0];
        let a = &doc["a"];
        let v = StringValidator::default();
        assert_eq!(v.validate(a), None);
        assert_eq!(v.validate(&doc["b"]), some_str!("field is not string"));

        let v = StringValidator {
            format: &NO_FORMAT,
            choices: hashset!("DOG".into(), "CAT".into()),
        };
        assert_eq!(v.validate(a), None);
        let v = StringValidator {
            format: &NO_FORMAT,
            choices: hashset!("DOG".into(), "FISH".into()),
        };
        let expected = some_str!("field is not one of {}", v.format_choices());
        assert_eq!(v.validate(a), expected);

        let v = StringValidator {
            format: IPV4_FORMAT,
            choices: HashSet::default(),
        };
        assert_eq!(v.validate(a), some_str!("field is not format of IPv4"));
        assert_eq!(v.validate(&doc["c"]), None);
    }
}
