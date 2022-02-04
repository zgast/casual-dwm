use yaml_rust::Yaml;
use crate::init::init_map::init_map;
use crate::file::file_loader::read_file;
use crate::file::file_writer::write_to_file;

pub fn check_config(config_vec: Vec<Yaml>){
    let path = "/home/markus/Documents/GitHub/casual-dwm/dwm/.xinitrc";
    let initrc_file =  read_file(path);

    let config = &config_vec[0];
    let features = init_map();
    let mut initrc: Vec<&str> = initrc_file.split("\n").clone().collect();


    let composing_activated = config["base"]["composing"].as_bool().unwrap();
    let background = features.get("background").unwrap().to_owned() + config["base"]["background"].as_str().unwrap();
    let auto_off = "auto_off_".to_owned() + config["base"]["auto_off"].as_str().unwrap();
    let spacer = "    SPACER=\"".to_owned() + config["bar"]["spacer"].as_str().unwrap() + "\"";
    let end = "    END=\"".to_owned() + config["bar"]["end"].as_str().unwrap() + "\"";
    let start = "    START=\"".to_owned() + config["bar"]["start"].as_str().unwrap() + "\"";

    let cpu_activated = config["bar"]["cpu"].as_bool().unwrap();
    let mem_activated = config["bar"]["ram"].as_bool().unwrap();
    let date_activated = config["bar"]["date"].as_bool().unwrap();
    let time_activated = config["bar"]["time"].as_bool().unwrap();


    for i in 0 .. initrc.len() {
        if initrc[i].eq(features.get("composing").unwrap()) && !composing_activated{
            initrc[i] = features.get("composing_deactivated").unwrap();
        }

        else if initrc[i].eq(features.get("composing_deactivated").unwrap()) && composing_activated{
            initrc[i] = features.get("composing").unwrap();
        }

        else if initrc[i].contains(features.get("background").unwrap()) && !initrc[i].eq(&background) {
            initrc[i] = &*background;
        }

        else if initrc[i].contains(features.get("auto_off_generic").unwrap()) && !initrc[i].eq(features.get(&auto_off).unwrap()){
            initrc[i] = features.get(&auto_off).unwrap();
        }

        else if cpu_activated && initrc[i].eq(features.get("cpu_off").unwrap()) {
            initrc[i] = features.get("cpu_on").unwrap()
        } else if !cpu_activated && initrc[i].eq(features.get("cpu_on").unwrap()) {
            initrc[i] = features.get("cpu_off").unwrap()
        }

        else if !mem_activated && initrc[i].eq(features.get("mem_on").unwrap()) {
            initrc[i] = features.get("mem_off").unwrap()
        }else if mem_activated && initrc[i].eq(features.get("mem_off").unwrap()) {
            initrc[i] = features.get("mem_on").unwrap()
        }

        else if !date_activated && initrc[i].eq(features.get("date_on").unwrap()) {
            initrc[i] = features.get("date_off").unwrap()
        }else if date_activated && initrc[i].eq(features.get("date_off").unwrap()) {
            initrc[i] = features.get("date_on").unwrap()
        }

        else if !time_activated && initrc[i].eq(features.get("time_on").unwrap()) {
            initrc[i] = features.get("time_off").unwrap()
        }else if time_activated && initrc[i].eq(features.get("time_off").unwrap()) {
            initrc[i] = features.get("time_on").unwrap()
        }

        else if initrc[i].contains("    SPACER=\"") && !initrc[i].contains(&*spacer){
            initrc[i] = &*spacer;
        }else if initrc[i].contains("    END=\"") && !initrc[i].contains(&*end){
            initrc[i] = &*end;
        }else if initrc[i].contains("    START=\"") && !initrc[i].contains(&*start){
            initrc[i] = &*start;
        }
    }

    let mut output = String::new();
    for line in initrc{
        output.push_str(line);
        output.push_str("\n");
    }

    write_to_file(path, output);
}