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


    map.insert(String::from("cpu_on"),String::from("    CPU=\"$(top -b -n1 | grep \"Cpu(s)\" | awk '{print $2 + $4}')% / 100%\""));
    map.insert(String::from("cpu_off"),String::from("    CPU=\"\""));


    map.insert(String::from("mem_on"),String::from("    MEM=\"$(free -m | grep '^Mem' | awk '{print  $3 \" MB / \" $2\" MB\"}')\""));
    map.insert(String::from("mem_off"),String::from("    MEM=\"\""));

    map.insert(String::from("date_on"),String::from("    DATE=\"$(date +\"%d.%m.%Y\")\""));
    map.insert(String::from("date_off"),String::from("    DATE=\"\""));

    map.insert(String::from("time_on"),String::from("    TIME=\"$(date \"+%R\")\""));
    map.insert(String::from("time_off"),String::from("    TIME=\"\""));

    return map;
}