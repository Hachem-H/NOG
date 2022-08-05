# NOG
NOG, or No-OS-Games, is a collection of old retro games capable of running without the use of an operating system as the games themself have a packaged kernel. It is built using the [rust programming language](https://rust-lang.org) using serveral external dependencies. Since writing kernels is quite a repetitive process, I abstracted away most the code in the `kernel/` library which deals with the low level stuff such as IDT/GDT, console output and clocks, to which the other applications use.

The first iteration of this project started off as a collection of bootsector games, where you could just assemble and boot the image, but that proved to be far more challenging than I first anticipated, so this what I decided to do instead as it is far easier since I am practically just writing terminal games in rust with a terminal interface. I must say, rust is certainly not my favorite language and stuff was extremely messy and quite hacky to say the least but I managed.

- [x] TicTacToe
- [x] Pong
- [x] Breakout

_more games to come perhaps?_

## Building and running the games
Since cargo have scripts which they can read from, everything is already taken care of for you so all we have to do is run the following commands:

```sh
# Run in QEMU
$ cargo run --release -p tictactoe 
$ cargo run --release -p pong 
$ cargo run --release -p breakout 
```

## Running on real hardware

I canno't hide the fact that I am extremely proud of the fact that all these binary images can infact run on real hardware _(a fact which I have tested my self)_.  There is numerous ways to generate a bootable USB, you can use [balena Etcher](https://www.balena.io/etcher/) if you don't want to use the terminal or if you are on windows. Otherwise, we can use the `dd` unix utility to generate this easily for us.

```sh
# Compile and flash to device
$ cargo bootimage --release -p tictactoe && sudo dd if=target/target/release/bootimage-tictactoe.bin of=/dev/[device]
$ cargo bootimage --release -p pong      && sudo dd if=target/target/release/bootimage-pong.bin of=/dev/[device]
$ cargo bootimage --release -p breakout  && sudo dd if=target/target/release/bootimage-breakout.bin of=/dev/[device]

# When you are done
$ sync
```

## Contribution

If you want to contribute to this project you most are most certain welcome to do so. Almost all contributions are welcome, from ideas to bugs and even possibly full game additions. This repository is licensed under the MIT license, so make to respect it's guidelines and rules.
