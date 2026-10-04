# bottom for Raspberry Pi 5

This fork of [ClementTsang/bottom](https://github.com/ClementTsang/bottom) adds:

- A `power` graph showing Raspberry Pi 5 PMIC rail power in watts.
- `hidden = true` on any widget in a custom layout. Hidden widgets are removed
  from navigation, remaining widgets fill the space, and unused collectors stop.

Power is the sum of the reported rail voltages multiplied by their currents.
It excludes USB/direct 5V loads and conversion losses, so it is not wall power.
Missing readings leave gaps and display “unavailable”.

## Build

On Raspberry Pi OS or another Linux system, install a C compiler/linker, Git,
and Rust 1.95 or newer (using [rustup](https://rustup.rs/) if needed).
For live power readings, `vcgencmd pmic_read_adc` must work on the Pi.

```sh
git clone https://github.com/procodr/bottom.git
cd bottom
cargo build --release --locked
```

The executable is `target/release/btm`. The initial release build can take a
while on a Pi. To run the library tests:

```sh
cargo test --lib --locked
```

## Install

Install for your user without replacing any package-managed files:

```sh
mkdir -p "$HOME/.local/bin"
install -m 755 target/release/btm "$HOME/.local/bin/btm"
export PATH="$HOME/.local/bin:$PATH"
hash -r
command -v btm
btm --version
```

Ensure `~/.local/bin` comes before `/snap/bin` and `/usr/bin` in your shell's
PATH. If needed, add the `export PATH` line above to `~/.profile`, then start a
new login session. This makes the new binary the default `btm`; a previous Snap
or distribution package remains available for rollback.

Install the included layout (backing up any existing config first):

```sh
mkdir -p "$HOME/.config/bottom"
if [ -f "$HOME/.config/bottom/bottom.toml" ]; then
    cp -p "$HOME/.config/bottom/bottom.toml" \
        "$HOME/.config/bottom/bottom.toml.backup-$(date +%Y%m%d-%H%M%S)"
fi
install -m 644 sample_configs/rpi5.toml "$HOME/.config/bottom/bottom.toml"
```

If you use `XDG_CONFIG_HOME`, put the config in `$XDG_CONFIG_HOME/bottom/`
instead, or select it explicitly with `btm -C /path/to/bottom.toml`.

## Run and configure

```sh
sudo -v
btm
```

`sudo -v` is needed only when `vcgencmd` requires root. Run it in the same
terminal as bottom. The collector first tries direct access, then
`sudo -n vcgencmd pmic_read_adc`; it never prompts inside the UI or stores a
password. Renew sudo authorization if power becomes unavailable.

Edit `~/.config/bottom/bottom.toml` and restart bottom to apply changes:

```toml
[[row]]
  [[row.child]]
  type = "power"
  [[row.child]]
  type = "disk"
  hidden = true
```

Set `hidden = false` or remove the setting to show a widget again. Keep at
least one widget visible. Custom layouts and power graphs are for normal mode,
not `--basic`. The power graph supports standard zoom, expand, and freeze controls.

To update after pulling changes, repeat the release build and binary install.
The general upstream documentation remains in [README.md](README.md).
