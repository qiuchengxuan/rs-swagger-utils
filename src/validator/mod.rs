pub trait Validator<T: ?Sized> {
    fn validate(&self, t: &T) -> Option<String>;
}

macro_rules! some_str {
    ($x:expr) => {
        Some($x.to_string())
    };
    ($x:expr, $($y:expr),+) => {
        Some(format!($x, $($y),+))
    };
}

pub struct UnknownValidator;

impl<T> Validator<T> for UnknownValidator {
    fn validate(&self, _: &T) -> Option<String> {
        return some_str!("Unknown type");
    }
}

mod array;
pub mod boolean;
mod common;
pub mod format;
pub mod integer;
mod object;
mod reference;
pub mod string;
