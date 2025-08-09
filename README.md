# OnionShot: Yet Another Screenshot Utility For Hyprland

`hyprshot` and `wlshot` doesn't do what I want to do so I wrote my own one.

## Compilation

Just `cargo build --release` and you're ready to go.

You can also install it using `cargo install --release`

## Dependencies

- `hyprland`
- `hyprpicker`
- `grim`
- `slurp`
- `wl-copy`
- `notify-send`

We check for every dependency's existence before running, so make sure you run onionshot at least once to see if some dependencies are missing. If so, the program will list out all missing programs and just install them and making sure they're accessible from `$PATH`.

## Environemnt Variables

`ONIONSHOT_DIR` is where we put the screenshots in (if the storage mode is `filesystem-only` or `both`). If this variable doesn't exist, then we will check for `$XDG_PICTURES_DIR`, and then `$HOME/Pictures`.

## Usage

The help (onionshot --help) is quite self-explanatory. We currently provides three modes (Fullscreen, region and active window) to screenshot with, and three storage modes (store to filesystem/clipboard only, or both). `-f` or `--freeze` can be use to freeze the screen in region mode and doesn't apply to fullscreen/active window mode because these two modes take screenshots instantly.

