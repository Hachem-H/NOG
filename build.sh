#!/usr/bin/env sh

mkdir -p images
cargo bootimage

cp target/target/debug/bootimage-tictactoe.bin images/TicTacToe.bin
cp target/target/debug/bootimage-pong.bin images/Pong.bin
