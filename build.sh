#!/usr/bin/env sh

mkdir -p images
cargo bootimage

cp target/target/debug/bootimage-tictactoe.bin images/TicTacToe.bin
