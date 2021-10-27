# Game of life using Fully homomorphic encryption

A simple implementation of Conway's Game of Life built using Zama's [concrete-boolean](https://github.com/zama-ai/concrete/tree/master/concrete-boolean) library.

## Build

Just run `cargo build --release`. 

## Run

Run `./target/release/homomorphic_game_of_life` (or `./target/release/homomorphic_game_of_life.exe` on Windows). 

## What does it do exactly?

This is a very simple implementation of Conway's [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) using periodic boundary conditions, with a twist: the board is encrypted using a fully homomorphic encryption scheme ([TFHE](https://eprint.iacr.org/2018/421.pdf)) and all calculations are performed in encrypted space. In principle, calculations can thus be done by a thread or server which has no access to the state of the board. The drawback, of course, is that it is also rather slow.
