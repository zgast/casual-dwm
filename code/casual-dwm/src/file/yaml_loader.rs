extern crate yaml_rust;

use yaml_rust::{Yaml, YamlLoader};
use crate::file::file_loader::read_file;

pub fn load_config(path:String) -> Vec<Yaml> {
    let  docs = YamlLoader::load_from_str(&*read_file(&*path)).unwrap();
    return docs;
}
