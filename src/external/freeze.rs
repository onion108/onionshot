use std::{
    process::{Child, Command},
    thread::sleep,
    time::Duration,
};

use super::hyprctl::{hide_cursor, reload};

pub struct FreezeHandle {
    child: Child,
}

pub fn freeze_screen() -> FreezeHandle {
    hide_cursor();
    let child = Command::new("wayfreeze")
        .arg("--hide-cursor")
        .spawn()
        .expect("failed to spawn hyprpicker");
    sleep(Duration::from_millis(100));
    reload();
    FreezeHandle { child }
}

impl Drop for FreezeHandle {
    fn drop(&mut self) {
        self.child
            .kill()
            .expect("failed to kill hyprpicker somehow");
    }
}
