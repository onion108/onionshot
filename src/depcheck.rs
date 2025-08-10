use std::process::{Child, Command, Stdio};

fn check_comp(child: Result<Child, std::io::Error>) -> bool {
    if let Ok(mut child) = child {
        match child.wait().map(|s| s.code().unwrap_or(-1)) {
            Ok(0) => true,
            _ => false,
        }
    } else {
        false
    }
}

struct Checker {
    name: &'static str,
    arg: &'static str,
}

fn checker(name: &'static str, arg: &'static str) -> Checker {
    Checker { name, arg }
}

pub fn check_dep() -> Option<Vec<&'static str>> {
    let mut missing_components = vec![];
    let checkers = vec![
        checker("hyprland", "--version"),
        checker("grim", "-h"),
        checker("slurp", "-h"),
        checker("hyprpicker", "-h"),
        checker("wl-copy", "--help"),
        checker("notify-send", "--help"),
    ];

    for checker in checkers {
        let child = Command::new(checker.name)
            .arg(checker.arg)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
        if !check_comp(child) {
            missing_components.push(checker.name);
        }
    }
    if missing_components.is_empty() {
        None
    } else {
        Some(missing_components)
    }
}
