use yaml_rust::Yaml;
use crate::init::bash_struct::bash_struct;
use std::fs;
use crate::file::file_loader::read_file;
use crate::file::file_writer::write_to_file;

pub fn check_config(config_vec: Vec<Yaml>){
    let path = "/home/markus/Documents/GitHub/casual-dwm/dwm/.initrc";
    let initrc_file =  read_file(path);

    let config = &config_vec[0];
    let mut features = bash_struct();
    let mut initrc: Vec<&str> = initrc_file.split("\n").clone().collect();

    let composing_activated = config["base"]["composing"].as_bool().unwrap();
    let background_path = config["base"]["background"].as_str().unwrap();

    for i in 0 .. initrc.len() {
        if initrc[i].eq(features.get("composing").unwrap()) && !composing_activated{
            initrc[i] = features.get("composing_deactivated").unwrap();
        }else if initrc[i].eq(features.get("composing_deactivated").unwrap()) && composing_activated{
            initrc[i] = features.get("composing").unwrap();
        }
    }

    println!("{:?}", initrc);

    let mut output = String::new();
    for line in initrc{
        output.push_str(line);
        output.push_str("\n");
    }

    write_to_file(path, output);
}