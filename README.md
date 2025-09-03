<img src="https://github.com/dekrammckraken/suprps/blob/main/suprps.png" alt="logo" width="200"/>

Press the PS button on your controller to launch your preferred game launcher. 
This tool was created for personal use to simplify startup in a window manager for those who prefer an ultra-minimal interface.

### Build a release

```sh
cargo build --release
cp ./target/release/suprps /your/desired/path
```

### Example with launch in Hyprland
Add this line to your Hyprland config: `exec-once = suprps`

**Make sure `suprps` is in your PATH.**

## See log
journalctl -e -t suprps -f



