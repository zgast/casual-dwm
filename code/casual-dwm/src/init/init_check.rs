use yaml_rust::Yaml;
use crate::init::bash_struct::bash_struct;
use std::fs;

pub fn check_config(config_vec: Vec<Yaml>){
    let config = &config_vec[0];
    let initrc_file = fs::read_to_string("/home/markus/Documents/GitHub/casual-dwm/dwm/.initrc").unwrap();
    let mut features = bash_struct();
    let mut initrc: Vec<&str> = initrc_file.split("\n").clone().collect();

    let composing_feature = features.get("composing").unwrap();
    let mut composing_bool = false;
    let composing_activated = config["base"]["composing"].as_bool().unwrap();

    let background_path = config["base"]["background"].as_str().unwrap();

    for line in initrc {
        if  composing_activated && !line.eq(composing_feature) && !composing_bool{
            output.push('\n');
            output.push_str(composing_feature);
            composing_bool = true;
        } else if !composing_activated && line.eq(composing_feature){
            line = "";
        }

        if !line.contains(background_path){

        }
    }


    println!("{}", output);

}