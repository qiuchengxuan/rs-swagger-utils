use std::collections::BTreeMap;

use common::Attribute;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum In {
    Path,
    FormData,
    Body,
    Header,
    Query,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub in_: In,
    #[serde(default)]
    pub required: bool,
    #[serde(flatten)]
    pub attribute: Attribute,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Method {
    Put,
    Post,
    Get,
    Patch,
    Delete,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Operation {
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Operations {
    pub put: Option<Operation>,
    pub post: Option<Operation>,
    pub get: Option<Operation>,
    pub patch: Option<Operation>,
    pub delete: Option<Operation>,
}

pub struct OperationsIter<'a> {
    operations: &'a Operations,
    next: Method,
    done: bool,
}

impl Operations {
    pub fn iter(&self) -> OperationsIter {
        OperationsIter {
            operations: &self,
            next: Method::Put,
            done: false,
        }
    }
}

impl<'a> Iterator for OperationsIter<'a> {
    type Item = (Method, &'a Operation);

    fn next(&mut self) -> Option<(Method, &'a Operation)> {
        loop {
            if self.done {
                return None;
            }
            let this = self.next;
            let option = match self.next {
                Method::Put => {
                    self.next = Method::Post;
                    &self.operations.put
                }
                Method::Post => {
                    self.next = Method::Get;
                    &self.operations.post
                }
                Method::Get => {
                    self.next = Method::Patch;
                    &self.operations.get
                }
                Method::Patch => {
                    self.next = Method::Delete;
                    &self.operations.patch
                }
                Method::Delete => {
                    self.done = true;
                    &self.operations.delete
                }
            };
            if option.is_some() {
                return Some((this, option.as_ref().unwrap()));
            }
        }
    }
}

pub type Paths = BTreeMap<String, Operations>;

pub mod uri;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::File;
    use std::path::PathBuf;

    use super::{Method, Operation, Paths};
    use path::uri::{Segment, SegmentIter};

    #[test]
    fn test_load_paths() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src/path/test.yaml");
        let file = File::open(path.to_str().unwrap()).unwrap();
        let root: HashMap<String, Paths> = serde_yaml::from_reader(&file).unwrap();
        let paths = root.get("paths").unwrap();
        let pet = paths.get("/pet/{petId}").unwrap();
        let get_pet = pet.get.as_ref().unwrap();
        let get_pet_parameters = get_pet.parameters.as_ref().unwrap();
        assert_eq!(get_pet_parameters[0].name, "petId");

        let operations: Vec<(&String, Method, &Operation)> = paths
            .iter()
            .flat_map(|(uri, v)| {
                v.iter()
                    .map(move |(method, operation)| (uri, method, operation))
            })
            .collect();
        let (uri, _, operation) = operations[0];
        let _segments: Vec<Segment> = SegmentIter::from((uri, operation)).collect();
    }
}
