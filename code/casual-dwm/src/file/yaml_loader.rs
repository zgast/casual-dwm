extern crate yaml_rust;

use std::fs;
use yaml_rust::{YamlLoader};

pub fn load_config(path:String){
    let docs = YamlLoader::load_from_str(&*read_file(path)).unwrap();
    let doc = &docs[0];

    println!("{:?}", doc);

}

fn read_file(path:String)  -> String{
    return fs::read_to_string(path).expect("Unable to read file");
}
