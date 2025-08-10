use std::{env, path::PathBuf};

pub fn screenshot_dir() -> PathBuf {
    env::var("ONIONSHOT_DIR")
        .or_else(|_| env::var("XDG_PICTURES_DIR"))
        .map(|x| x.into())
        .unwrap_or(env::home_dir().unwrap_or("/".into()).join("Pictures"))
}

pub fn ensure_screenshot_dir() {
    if !screenshot_dir().exists() {
        std::fs::create_dir_all(screenshot_dir()).expect("failed to create screenshot directories");
    }
}
