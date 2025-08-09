use std::{path::Path, process::Command};

pub fn notify_screenshot_save(screenshot_path: &Path) {
    Command::new("notify-send")
        .arg("-i")
        .arg(screenshot_path.to_string_lossy().to_string())
        .arg("-a")
        .arg("onionshot")
        .arg("Screenshot saved")
        .arg(format!(
            "Screenshot successfully saved to {}",
            screenshot_path.to_string_lossy().to_string()
        ))
        .output()
        .expect("failed to send notify. Oops! ");
}

pub fn notify_clipboard_save() {
    Command::new("notify-send")
        .arg("-a")
        .arg("onionshot")
        .arg("Screenshot saved")
        .arg("Screenshot successfully saved to clipboard")
        .output()
        .expect("failed to send notify. Oops! ");
}

pub fn notify_save_fail(error: std::io::Error) {
    Command::new("notify-send")
        .arg("-a")
        .arg("onionshot")
        .arg("Screenshot saving failed")
        .arg(format!("Screenshot failed to save: {}", error))
        .output()
        .expect("failed to send notify. Oops! ");
}
