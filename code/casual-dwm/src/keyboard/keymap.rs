use std::collections::HashMap;
pub fn keymap() -> HashMap<String, String> {
    let mut map = HashMap::new();
    let alphabet = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"
                               ,"0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
                                "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10",
                                "minus", "space", "equal", "comma", "period", "backslash"];

    for char in alphabet {
        map.insert(String::from(char), String::from("XK_" + char));
    }

    map.insert(String::from("super"),String::from("XK_Super_L"));
    map.insert(String::from("control"),String::from("XK_Control_L"));
    map.insert(String::from("enter"),String::from("XK_"));
    map.insert(String::from("alt"),String::from("XK_Alt_L"));
    map.insert(String::from("alt_gr"),String::from("XK_Alt_R"));

    map.insert(String::from("shift_l"),String::from("XK_Shift_L"));
    map.insert(String::from("shift_r"),String::from("XK_Shift_R"));
    map.insert(String::from("shift_lock"),String::from("XK_Shift_Lock"));


    map.insert(String::from("return"),String::from("XK_Return"));
    map.insert(String::from("escape"),String::from("XK_Escape"));
    map.insert(String::from("tab"),String::from("XK_Tab"));

    map.insert(String::from("arrow_up"),String::from("XK_Up"));
    map.insert(String::from("arrow_down"),String::from("XK_Down"));
    map.insert(String::from("arrow_left"),String::from("XK_Left"));
    map.insert(String::from("arrow_right"),String::from("XK_Right"));

    return map;
}
