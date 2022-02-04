use std::collections::HashMap;
pub fn key_command_map() -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert(String::from("quit_dwm"),String::from("picom -f &"));
    map.insert(String::from("quit_window"),String::from("picom -f &"));
    map.insert(String::from("shift_window_master"),String::from("picom -f &"));
    map.insert(String::from("change_window_tab"),String::from("picom -f &"));

    map.insert(String::from("rofi"),String::from("picom -f &"));
    map.insert(String::from("lock"),String::from("picom -f &"));
    map.insert(String::from("screenshot"),String::from("picom -f &"));

    map.insert(String::from("tiled"),String::from("picom -f &"));
    map.insert(String::from("floating"),String::from("picom -f &"));
    map.insert(String::from("grid"),String::from("picom -f &"));
    map.insert(String::from("monocle"),String::from("picom -f &"));

    map.insert(String::from("master_minus"),String::from("picom -f &"));
    map.insert(String::from("master_plus"),String::from("picom -f &"));

    return map;
}