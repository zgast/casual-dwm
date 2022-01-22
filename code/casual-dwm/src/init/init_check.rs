use yaml_rust::Yaml;
use crate::init::bash_struct::bash_struct;
use std::fs;

pub fn check_config(config_vec: Vec<Yaml>){
    let config = &config_vec[0];
    let initrc_file = fs::read_to_string("/home/markus/Documents/GitHub/casual-dwm/dwm/.initrc").unwrap();
    let features = bash_struct();

    let mut output = initrc_file.clone();
    if config["base"]["composing"].as_bool().unwrap() {
        let composing_feature = features.get("composing").unwrap();
        let mut composing_bool = false;
        for line in initrc_file.lines() {
            if !line.eq(features.get("composing").unwrap()) && !composing_bool{
                output.push('\n');
                output.push_str(composing_feature);
                composing_bool = true;
            }
        }
    }

    println!("{}", output);

}