use common::Attribute;
use std::collections::HashMap;

pub type Definitions = HashMap<String, Attribute>;

#[cfg(test)]
mod tests {
    use common::TypeDefinition;
    use definition::Definitions;
    use std::collections::HashMap;
    use std::fs::File;
    use std::path::PathBuf;

    #[test]
    fn test_load_definitions() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src/definition/test.yaml");
        let file = File::open(path.to_str().unwrap()).unwrap();
        let root: HashMap<String, Definitions> = serde_yaml::from_reader(&file).unwrap();
        let definitions = root.get("definitions").unwrap();

        let category = definitions.get("Category").unwrap();
        let category_def = category.definition.as_ref().unwrap();
        match category_def {
            TypeDefinition::Object(_) => (),
            _ => panic!("Not matched"),
        };

        let pet = definitions.get("Pet").unwrap();
        let pet_def = pet.definition.as_ref().unwrap();
        let pet_props = match pet_def {
            TypeDefinition::Object(t) => &t.properties,
            _ => panic!("Not matched"),
        };

        let photo_urls = pet_props.get("photoUrls").unwrap();
        let photo_urls_def = photo_urls.definition.as_ref().unwrap();
        match photo_urls_def {
            TypeDefinition::Array { items: _ } => (),
            _ => panic!("Not matched"),
        }
    }
}
