use std::collections::HashMap;
pub fn keymap() -> HashMap<String, String> {
    let mut map = HashMap::new();
    let alphabet = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];

    for char in alphabet {
        map.insert(String::from(char), String::from("XK_" + char));
    }

    map.insert(String::from("super"),String::from("XK_"));


    return map;
}
