# RSCONFIG-macros
[<img alt="github" src="https://img.shields.io/github/last-commit/hypercodec/rsconfig-macros" height="20">](https://github.com/hypercodec/rsconfig-macros)
[<img alt="crates.io" src="https://img.shields.io/crates/d/rsconfig-macros" height="20">](https://crates.io/crates/rsconfig-macros)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/rsconfig-macros" height="20">](https://docs.rs/rsconfig-macros)

Macro implementation for RSCONFIG

## Examples
### FileConfig
```rust
#[derive(Debug, FileConfig)]
struct TestConfig {
    test: bool
}
impl YamlConfig for TestConfig {
    fn from_yaml(yaml: Vec<yaml_rust::Yaml>) -> Self {
        Self { test: *&yaml[0]["test"].as_bool().unwrap() }
    }
    fn save_yaml(&self, path: &str) -> Result<()> {
        let mut data = "test: ".to_string();
        data.push_str(self.test.to_string().as_str());
        fs::write(path, data).unwrap();

        Ok(())
    }
}
impl JsonConfig for TestConfig {
    fn from_json(val: Value) -> Self {
        Self { test: val["test"].as_bool().unwrap() }
    }
    fn save_json(&self, path: &str) -> io::Result<()> {
        // convert to json pretty format and save
        let mut m: HashMap<&str, Value> = HashMap::new();
        m.insert("test", Value::from(self.test));
        let data = serde_json::to_string_pretty(&m).unwrap();
        fs::write(path, data).unwrap();
     
        Ok(())
    }
}
```