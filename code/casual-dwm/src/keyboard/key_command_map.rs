use std::collections::HashMap;
pub fn key_command_map() -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert(String::from("quit_dwm"),String::from("quit,           {0} "));
    map.insert(String::from("quit_window"),String::from("killclient,     {0}"));
    map.insert(String::from("shift_window_master"),String::from("zoom, 			{0}"));
    map.insert(String::from("focus_window"),String::from("setlayout,      {0}"));
    map.insert(String::from("change_window_focus"),String::from("focusstack,     {.i = +1 }"));
    map.insert(String::from("toggle_bar"),String::from("togglebar,      {0}"));

    map.insert(String::from("application_select"),String::from("SHCMD(\"rofi -modi drun -show drun -hide-scrollbar -show-icons -drun-icon-theme\")"));
    map.insert(String::from("lock"),String::from("SHCMD(\"env XSECURELOCK_PASSWORD_PROMPT=asterisks env XSECURELOCK_SHOW_DATETIME=1 env XSECURELOCK_SAVER=saver_mpv env XSECURELOCK_LIST_VIDEOS_COMMAND='feh --zoom=fill -F ~/seafile/MyLibrary/pics/backgrounds/space.jpg' xsecurelock\")"));
    map.insert(String::from("screenshot"),String::from("SHCMD(\"scrot -u ~/Screenshot/%s_%d.%m_%R.png\")"));

    map.insert(String::from("tiled"),String::from("  setlayout,      {.v = &layouts[0]}"));
    map.insert(String::from("floating"),String::from("setlayout,      {.v = &layouts[1]}"));
    map.insert(String::from("grid"),String::from(" setlayout,      {.v = &layouts[3]}"));
    map.insert(String::from("monocle"),String::from("setlayout,      {.v = &layouts[2]}"));

    map.insert(String::from("master_minus"),String::from("setmfact,       {.f = -0.05} "));
    map.insert(String::from("master_plus"),String::from("setmfact,       {.f = +0.05} "));
    return map;
}