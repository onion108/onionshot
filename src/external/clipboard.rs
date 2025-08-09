use std::{fs::OpenOptions, io::{Read, Write}, path::Path, process::{Command, Stdio}};

pub fn copy_png(path: &Path) -> std::io::Result<()> {
    let mut pngdata = Vec::new();
    OpenOptions::new().read(true).open(path)?.read_to_end(&mut pngdata)?;
    let mut wlcopy = Command::new("wl-copy").arg("--type").arg("image/png").stdin(Stdio::piped()).spawn().expect("failed to spawn wl-copy");
    let Some(mut input) = wlcopy.stdin else {
        wlcopy.kill().unwrap();
        panic!("failed to obtain stdin of wl-copy");
    };
    input.write_all(&pngdata)?;
    drop(input);
    Ok(())
}
