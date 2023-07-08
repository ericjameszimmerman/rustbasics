
# Rust + GTK4

## Install and Basic Example
```
# -------------
# GTK4 install
# -------------

# Set Rust toolchain to MSVC
# Note that Visual Studio 2022 Community was already installed on my system
rustup default stable-msvc

# install build dependencies
python -m pip install --user pipx
python -m pipx ensurepath
pipx install gvsbuild

# Follow the gvsbuild docs to build GTK 4. 
# https://github.com/wingtk/gvsbuild#development-environment
gvsbuild build gtk4

# add the following to PATH
C:\gtk-build\gtk\x64\release\bin

# -------------
# project setup
# -------------
cargo new hellorustgtk4
cd hellorustgtk4
code .

# Find out the GTK 4 version on your machine by running
# it returned "4.10.4"
pkg-config --modversion gtk4

# Use this information to add the gtk4 crate to your dependencies in Cargo.toml. At the time of this writing the newest version is 4.10.
cargo add gtk4 --rename gtk --features v4_10

# build
cargo build

# build and run
cargo run

```

## Drag and Drop Text View Example

```bash
cargo add gio
cargo add gdk
```