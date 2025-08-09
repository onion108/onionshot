use super::common::Geometry;
use std::{
    process::{Command, Stdio},
    thread,
    time::Duration,
};

pub fn get_active_window() -> Geometry {
    let output = Command::new("hyprctl")
        .arg("-j")
        .arg("activewindow")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to spawn hyprctl");
    if !output.status.success() {
        panic!("Failed to execute hyprctl");
    }
    let Ok(data) = str::from_utf8(&output.stdout) else {
        panic!("non-utf8 fuck off");
    };

    let obj = json::parse(data).expect("hyprctl -j not returning json");
    if !obj.is_object() {
        panic!("failed to parse hyprctl's active window output: not an object at top level");
    }
    if !obj.has_key("at") {
        panic!("failed to parse hyprctl's active window output: property `at' not found");
    }
    if !obj.has_key("size") {
        panic!("failed to parse hyprctl's active window output: property `size' not found");
    }

    let at = &obj["at"];
    let size = &obj["size"];

    if !(at.is_array() && at.len() == 2 && at.members().fold(true, |m, x| m && x.is_number())) {
        panic!(
            "failed to parse hyprctl's active window output: property `at' isn't an array of two numbers"
        );
    }
    if !(size.is_array() && size.len() == 2 && size.members().fold(true, |m, x| m && x.is_number()))
    {
        panic!(
            "failed to parse hyprctl's active window output: property `at' isn't an array of two numbers"
        );
    }
    Geometry {
        x: at[0].as_i32().unwrap(),
        y: at[1].as_i32().unwrap(),
        w: size[0].as_u32().unwrap(),
        h: size[1].as_u32().unwrap(),
    }
}

pub fn get_scale() -> f32 {
    let output = Command::new("hyprctl")
        .arg("-j")
        .arg("monitors")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to spawn hyprctl");
    if !output.status.success() {
        panic!("Failed to execute hyprctl");
    }
    let Ok(data) = str::from_utf8(&output.stdout) else {
        panic!("non-utf8 fuck off");
    };

    let obj = json::parse(data).expect("hyprctl -j not returning json");
    if !obj.is_array() {
        panic!("failed to parse hyprctl's monitors' output: not an array at top level");
    }

    if !obj.members().fold(true, |m, x| {
        m && x.is_object() && x.has_key("scale") && x.has_key("focused")
    }) {
        panic!(
            "failed to parse hyprctl's monitors' output: top level is not an array of objects that have key `scale' and `focused'"
        );
    }
    obj.members()
        .find(|x| x["focused"].as_bool().unwrap_or(false))
        .expect("failed to get focused monitor: no monitors are focused!?")["scale"]
        .as_f32()
        .expect("failed to parse hyprctl's monitors' output: field `scale' isn't a number")
}

pub fn hide_cursor() {
    Command::new("hyprctl")
        .args(&["keyword", "cursor:inactive_timeout", "0.1"])
        .spawn()
        .expect("failed to spawn hyprctl")
        .wait()
        .expect("failed to run hyprctl");
    thread::sleep(Duration::from_millis(100));
}

pub fn reload() {
    Command::new("hyprctl")
        .arg("reload")
        .spawn()
        .expect("failed to spawn hyprctl")
        .wait()
        .expect("failed to run hyprctl");
}
