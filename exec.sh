#!/bin/bash
cargo build --package tetris-rs --bin tetris-rs && gnome-terminal -- ./target/debug/tetris-rs
exit