use std::{
    process::{Child, Command},
    thread::sleep,
    time::Duration,
};

pub struct FreezeHandle {
    child: Child,
}

pub fn freeze_screen() -> FreezeHandle {
    let child = Command::new("wayfreeze")
        .arg("--hide-cursor")
        .spawn()
        .expect("failed to spawn wayfreeze");
    sleep(Duration::from_millis(100));
    FreezeHandle { child }
}

impl Drop for FreezeHandle {
    fn drop(&mut self) {
        self.child.kill().expect("failed to kill wayfreeze somehow");
    }
}
