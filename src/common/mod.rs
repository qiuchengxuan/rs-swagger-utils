use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Deserialize)]
pub struct IntegerType {
    pub format: Option<String>,
    pub minimum: Option<i64>,
    pub maximum: Option<i64>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct StringType {
    pub format: Option<String>,
    #[serde(rename = "enum")]
    pub choices: Option<HashSet<String>>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ObjectType {
    pub properties: HashMap<String, Attribute>,
    pub required: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum TypeDefinition {
    Boolean,
    Integer(IntegerType),
    String(StringType),
    Array {
        items: Box<Attribute>,
    },
    Object(ObjectType),
    File,
    #[serde(skip_deserializing)]
    #[allow(dead_code)]
    Undefined,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Attribute {
    #[serde(default = "TypeDefinition::Undefined", flatten)]
    pub definition: Option<TypeDefinition>,
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
    pub description: Option<String>,
}
