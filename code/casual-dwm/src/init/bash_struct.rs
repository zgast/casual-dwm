use std::collections::HashMap;
pub fn bash_struct() -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert(String::from("composing"),String::from("picom -f &"));
    map.insert(String::from("background"),String::from("feh --bg-scale "));
    return map;
}