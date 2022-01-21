mod file;

use file::yaml_loader;

fn main() {
    yaml_loader::load_config("/home/markus/Documents/GitHub/casual-dwm/config/config.yml".to_string());
}
