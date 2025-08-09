use clap::{Parser, ValueEnum};

/// A screenshot utility program for Hyprland.
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct ApplicationArgs {
    #[arg(short, long, default_value = "fullscreen")]
    pub mode: Mode,

    /// Freeze the screen
    #[arg(short, long)]
    pub freeze: bool,

    /// Storage mode
    #[arg(short, long, default_value = "both")]
    pub storage: StorageMode,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Mode {
    Fullscreen,
    Region,
    ActiveWindow,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum StorageMode {
    FilesystemOnly,
    ClipboardOnly,
    Both,
}

