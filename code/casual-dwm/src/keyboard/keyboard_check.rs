use yaml_rust::Yaml;
use crate::init::init_map::init_map;
use crate::file::file_loader::read_file;
use crate::file::file_writer::write_to_file;
use crate::keyboard::key_command_map::key_command_map;
use crate::keyboard::keymap::keymap;

pub fn check_config(config_vec: Vec<Yaml>){
    let path = "/home/markus/Documents/GitHub/casual-dwm/dwm/.xinitrc";
    let config_h =  read_file(path);

    let config = &config_vec[0];
    let keymap = keymap();
    let commands = key_command_map();
    let mut config_h_lines: Vec<&str> = config_h.split("\n").clone().collect();
    let mut in_range:bool = false;

    for i in 0 .. config_h_lines.len() {
        for command in commands {

            }
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