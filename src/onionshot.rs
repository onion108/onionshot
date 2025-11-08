use std::{env::temp_dir, fs::OpenOptions, io::BufReader, path::Path};

use chrono::{Datelike, Timelike};

use crate::{
    argparse::{ApplicationArgs, StorageMode},
    env::{ensure_screenshot_dir, screenshot_dir},
    external::{
        clipboard::copy_png,
        common::Geometry,
        freeze::freeze_screen,
        grim::{grim, grim_with_geometry},
        hyprctl::{get_active_window, get_scale},
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
    grim(&tmppath);
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
        let child = grim(&tmppath);
        let _f = freeze_screen();
        let Some(geometry) = slurp_geometry() else {
            _ = std::fs::remove_file(&tmppath);
            return;
        };
        let _ = child.wait_with_output().expect("grim failed");
        let scale = get_scale();
        let actual_geometry = Geometry {
            x: (geometry.x as f32 * scale) as i32,
            y: (geometry.y as f32 * scale) as i32,
            w: (geometry.w as f32 * scale) as u32,
            h: (geometry.h as f32 * scale) as u32,
        };
        let Ok(grimmed) = OpenOptions::new().read(true).open(&tmppath) else {
            _ = std::fs::remove_file(&tmppath);
            return;
        };
        if let Err(_) = image::load(BufReader::new(grimmed), image::ImageFormat::Png)
            .map(|x| {
                x.crop_imm(
                    actual_geometry.x as u32,
                    actual_geometry.y as u32,
                    actual_geometry.w,
                    actual_geometry.h,
                )
            })
            .and_then(|x| x.save(&tmppath))
        {
            _ = std::fs::remove_file(&tmppath);
            return;
        }
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
