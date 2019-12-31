use derive_more::Display;

pub trait FormatValidator<T>: std::fmt::Display {
    fn validate(&self, _: T) -> bool;
}

#[derive(Display)]
#[display(fmt = "unknown")]
pub struct UnknownFormatValidator;

impl<T> FormatValidator<T> for UnknownFormatValidator {
    fn validate(&self, _: T) -> bool {
        false
    }
}

pub const UNKNOWN_FORMAT: UnknownFormatValidator = UnknownFormatValidator {};

#[derive(Display)]
#[display(fmt = "text")]
pub struct NoFormatValidator;

impl<T> FormatValidator<T> for NoFormatValidator {
    fn validate(&self, _: T) -> bool {
        true
    }
}

pub const NO_FORMAT: NoFormatValidator = NoFormatValidator {};
