#![warn(missing_docs)]

//! Macro implementation for RSCONFIG

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// Implements the default empty FileConfig if the required filetypes exist
/// ### Example
/// ```rust
/// # use rsconfig_macros::FileConfig;
/// # use std::collections::HashMap;
/// # use serde_json::{self, Value};
/// # use yaml_rust;
/// # use rsconfig::*;
/// # use std::io::{self, Result};
/// # use std::fs;
/// 
/// // FileConfig derive macro being used instead of adding `impl FileConfig for TestConfig {}` at the end
/// #[derive(Debug, FileConfig)]
/// struct TestConfig {
///     test: bool
/// }
///
/// impl YamlConfig for TestConfig {
///     fn from_yaml(yaml: Vec<yaml_rust::Yaml>) -> Self {
///         Self { test: *&yaml[0]["test"].as_bool().unwrap() }
///    }
///
///     fn save_yaml(&self, path: &str) -> Result<()> {
///         let mut data = "test: ".to_string();
///         data.push_str(self.test.to_string().as_str());
///
///         fs::write(path, data).unwrap();
///
///         Ok(())
///     }
/// }
///
/// impl JsonConfig for TestConfig {
///     fn from_json(val: Value) -> Self {
///         Self { test: val["test"].as_bool().unwrap() }
///     }
///
///     fn save_json(&self, path: &str) -> Result<()> {
///         // convert to json pretty format and save
///         let mut m: HashMap<&str, Value> = HashMap::new();
///         m.insert("test", Value::from(self.test));
///         let data = serde_json::to_string_pretty(&m).unwrap();
///         fs::write(path, data).unwrap();
///      
///         Ok(())
///     }
/// }
/// ```
#[proc_macro_derive(FileConfig)]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let name = syn::parse_macro_input!(input as DeriveInput).ident;

    TokenStream::from(quote! { impl FileConfig for #name {} })
}
