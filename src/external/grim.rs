use std::path::Path;
use std::process::Command;

use super::common::Geometry;

pub fn grim(output: &Path) {
    let _ = Command::new("grim")
        .arg(output.to_string_lossy().to_string())
        .output()
        .expect("failed to spawn grim");
}

pub fn grim_with_geometry(output: &Path, geometry: Geometry) {
    let _ = Command::new("grim")
        .arg("-g")
        .arg(format!(
            "{},{} {}x{}",
            geometry.x, geometry.y, geometry.w, geometry.h
        ))
        .arg(output.to_string_lossy().to_string())
        .output()
        .expect("failed to spawn grim");
}
