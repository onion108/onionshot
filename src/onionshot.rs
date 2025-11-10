use std::{env::temp_dir, path::Path};

use chrono::{Datelike, Timelike};

use crate::{
    argparse::{ApplicationArgs, StorageMode},
    env::{ensure_screenshot_dir, screenshot_dir},
    external::{
        clipboard::copy_png,
        freeze::freeze_screen,
        grim::grim_with_geometry,
        hyprctl::{get_active_screen, get_active_window},
        notify::{notify_clipboard_save, notify_save_fail, notify_screenshot_save},
        slurp::slurp_geometry,
    },
};

fn generate_image_name() -> String {
    let current_time = chrono::Local::now();
    format!(
        "{:04}-{:02}-{:02}-{:02}{:02}{:02}_onionshot.png",
        current_time.year(),
        current_time.month(),
        current_time.day(),
        current_time.hour(),
        current_time.minute(),
        current_time.second()
    )
}

fn save_image(from: &Path, to: &Path, mode: StorageMode) {
    match mode {
        StorageMode::FilesystemOnly => {
            if let Err(err) = std::fs::copy(from, to) {
                notify_save_fail(err);
            } else {
                notify_screenshot_save(to);
            }
        }
        StorageMode::ClipboardOnly => {
            if let Err(err) = copy_png(from) {
                notify_save_fail(err);
            } else {
                notify_clipboard_save();
            }
        }
        StorageMode::Both => {
            if let Err(err) = std::fs::copy(from, to) {
                notify_save_fail(err);
            } else {
                notify_screenshot_save(to);
            }
            if let Err(err) = copy_png(from) {
                notify_save_fail(err);
            } else {
                notify_clipboard_save();
            }
        }
    }
    _ = std::fs::remove_file(from);
}

pub fn fullscreen_shot(args: &ApplicationArgs) {
    ensure_screenshot_dir();
    let name = generate_image_name();
    let picpath = screenshot_dir().join(&name);
    let tmppath = temp_dir().join(&name);
    let geometry = get_active_screen();
    grim_with_geometry(&tmppath, geometry);
    save_image(&tmppath, &picpath, args.storage);
}

pub fn active_window_shot(args: &ApplicationArgs) {
    ensure_screenshot_dir();
    let name = generate_image_name();
    let picpath = screenshot_dir().join(&name);
    let tmppath = temp_dir().join(&name);

    let geometry = get_active_window();
    grim_with_geometry(&tmppath, geometry);
    save_image(&tmppath, &picpath, args.storage);
}

pub fn region_shot(args: &ApplicationArgs) {
    ensure_screenshot_dir();
    let name = generate_image_name();
    let picpath = screenshot_dir().join(&name);
    let tmppath = temp_dir().join(&name);

    if args.freeze {
        let f = freeze_screen();
        let Some(geometry) = slurp_geometry() else {
            return;
        };
        grim_with_geometry(&tmppath, geometry);
        drop(f);
        save_image(&tmppath, &picpath, args.storage);
    } else {
        let Some(geometry) = slurp_geometry() else {
            _ = std::fs::remove_file(&tmppath);
            return;
        };
        grim_with_geometry(&tmppath, geometry);
        save_image(&tmppath, &picpath, args.storage);
    }
}
