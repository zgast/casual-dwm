use std::collections::HashMap;
pub fn bash_struct() -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert(String::from("composing"),String::from("picom -f &"));
    map.insert(String::from("composing_deactivated"),String::from("# picom -f &"));

    map.insert(String::from("background"),String::from("feh --bg-scale "));

    map.insert(String::from("auto_off_fast"),String::from("xset dpms 20 80 100"));
    map.insert(String::from("auto_off_mid"),String::from("xset dpms 60 120 180"));
    map.insert(String::from("auto_off_slow"),String::from("xset dpms 100 200 260"));
    map.insert(String::from("auto_off_off"),String::from(" # xset dpms 100 200 260"));
    map.insert(String::from("auto_off_generic"),String::from("xset dpms"));
    return map;
}