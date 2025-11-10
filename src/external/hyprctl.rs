use super::common::Geometry;
use std::process::{Command, Stdio};

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
        panic!(
            "failed to parse hyprctl's active window output: not an object at top level"
        );
    }
    if !obj.has_key("at") {
        panic!(
            "failed to parse hyprctl's active window output: property `at' not found"
        );
    }
    if !obj.has_key("size") {
        panic!(
            "failed to parse hyprctl's active window output: property `size' not found"
        );
    }

    let at = &obj["at"];
    let size = &obj["size"];

    if !(at.is_array()
        && at.len() == 2
        && at.members().fold(true, |m, x| m && x.is_number()))
    {
        panic!(
            "failed to parse hyprctl's active window output: property `at' isn't an array of two numbers"
        );
    }
    if !(size.is_array()
        && size.len() == 2
        && size.members().fold(true, |m, x| m && x.is_number()))
    {
        panic!(
            "failed to parse hyprctl's active window output: property `size' isn't an array of two numbers"
        );
    }
    Geometry {
        x: at[0].as_i32().unwrap(),
        y: at[1].as_i32().unwrap(),
        w: size[0].as_u32().unwrap(),
        h: size[1].as_u32().unwrap(),
    }
}

pub fn get_active_screen() -> Geometry {
    // Get active output
    let output = Command::new("hyprctl")
        .arg("-j")
        .arg("activeworkspace")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to spawn hyprctl");
    if !output.status.success() {
        panic!("Failed to execute hyprctl");
    }
    let Ok(data) = str::from_utf8(&output.stdout) else {
        panic!("non-utf8 fuck off");
    };

    let workspace_obj =
        json::parse(data).expect("hyprctl -j not returning json");
    let monitor_id = workspace_obj["monitorID"]
        .as_usize()
        .expect("failed to parse hyprctl's active workspace output");

    // Get active output's bound
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

    let monitor =
        &json::parse(data).expect("hyprctl -j not returning json")[monitor_id];

    let msg = "failed to parse hyprctl's monitor output";

    let scale = monitor["scale"].as_f32().expect(msg);
    let x = monitor["x"].as_i32().expect(msg);
    let y = monitor["y"].as_i32().expect(msg);
    let w = monitor["width"].as_f32().expect(msg) / scale;
    let h = monitor["height"].as_f32().expect(msg) / scale;

    Geometry {
        x,
        y,
        w: w as u32,
        h: h as u32,
    }
}
