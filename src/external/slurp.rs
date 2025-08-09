use std::process::{Command, Stdio};

use super::common::Geometry;


pub fn slurp_geometry() -> Option<Geometry> {
    let output = Command::new("slurp")
        .arg("-d")
        .arg("-f")
        .arg("%x %y %w %h")
        .stdout(Stdio::piped())
        .output()
        .expect("failed to spawn slurp");
    if !output.status.success() {
        return None
    }

    let data = str::from_utf8(&output.stdout).expect("there shouldn't be anything non-utf8 in the output... right? right?? right???");
    let nums = data.trim().split(' ').map(|x| x.parse::<i64>().expect("invalid output from slurp... what happened?")).collect::<Vec<_>>();
    if nums.len() != 4 {
        panic!("slurp doesn't output 4 numbers");
    }
    Some(Geometry {
        x: nums[0] as i32,
        y: nums[1] as i32,
        w: nums[2] as u32,
        h: nums[3] as u32,
    })
}

