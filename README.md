# fzf_gui
A simple fuzzy finder for Wayland
(Tested on Sway version: 1.8.1)

**NOTE:** Requires support for [XDG Shell Protocol](https://wayland.app/protocols/xdg-shell) from your compositor.

![image](https://github.com/ArchUsr64/fzf_gui/assets/83179501/5f1fc69f-a09e-4d70-b1ec-06067dd78fe8)
## How to Use
1. Checkout: `git clone https://github.com/ArchUsr64/fzf_gui`
2. Build: `cargo build --release`
3. Copy to `$PATH`: `cp target/release/fzf_gui /usr/bin/`
4. Run: `ls | fzf_gui`
