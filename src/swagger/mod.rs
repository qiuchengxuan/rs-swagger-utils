use definition::Definitions;
use path::Paths;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Swagger {
    #[serde(rename = "swagger")]
    pub spec: Option<String>,
    pub definitions: Option<Definitions>,
    pub paths: Option<Paths>,
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::PathBuf;
    use swagger::Swagger;

    #[test]
    fn load_test_yaml() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src/swagger/test.yaml");
        let file = File::open(path.to_str().unwrap()).unwrap();
        let root: Swagger = serde_yaml::from_reader(&file).unwrap();
        assert!(root.paths.unwrap().len() > 0);
        assert!(root.definitions.unwrap().len() > 0);
    }
}
