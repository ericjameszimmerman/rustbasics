
# Rust + QT6

## Install and Basic Example
```
# -------------
# project setup
# -------------
cargo new hellorustqtcargo
cd hellorustqtcargo
code .

cargo add cxx
cargo add cxx-qt
cargo add cxx-qt-lib
cargo add cxx-qt-build

# verify QT build
qmake --version

# build and run
cargo run
