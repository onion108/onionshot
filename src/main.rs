use clap::Parser;
use onionshot::{argparse::{ApplicationArgs, Mode}, depcheck::check_dep, onionshot::{active_window_shot, fullscreen_shot, region_shot}};

fn main() {
    if cfg!(not(debug_assertions)) {
        std::panic::set_hook(Box::new(|info| {
            eprint!("\x1b[31mFATAL\x1b[00: ");
            if let Some(msg) = info.payload().downcast_ref::<&str>() {
                eprintln!("{msg}");
            } else if let Some(msg) = info.payload().downcast_ref::<String>() {
                eprintln!("{msg}");
            } else {
                eprintln!("<Some magic payload that no one understands. >")
            }
        }));
    }

    if let Some(missing) = check_dep() {
        if missing.len() == 1 {
            eprintln!("Missing dependency: {}", missing[0]);
        } else {
            eprintln!("Missing dependencies: {}", missing.join(", "));
        }
        return
    }

    let args = ApplicationArgs::parse();

    match args.mode {
        Mode::Fullscreen => {
            fullscreen_shot(&args);
        }
        Mode::ActiveWindow => {
            active_window_shot(&args);
        }
        Mode::Region => {
            region_shot(&args);
        }
    }
}
