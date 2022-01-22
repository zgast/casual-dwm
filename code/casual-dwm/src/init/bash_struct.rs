use std::collections::HashMap;
pub fn bash_struct() -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert(String::from("composing"),String::from("picom -f &"));
    return map;
}