mod file;
mod init;
mod keyboard;

use file::yaml_loader;
use init::init_check;
use crate::keyboard::keyboard_check;

fn main() {
    let init_config = yaml_loader::load_config("/home/markus/Documents/GitHub/casual-dwm/config/config.yml".to_string());
    let keyboard_config = yaml_loader::load_config("/home/markus/Documents/GitHub/casual-dwm/config/keyboard.yml".to_string());

    init_check::check_config(init_config);
    keyboard_check::check_config(keyboard_config);
}