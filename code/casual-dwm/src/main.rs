mod file;
mod init;

use file::yaml_loader;
use init::init_check;

fn main() {
    let config = yaml_loader::load_config("/home/markus/Documents/GitHub/casual-dwm/config/config.yml".to_string());
    init_check::check_config(config)
}
