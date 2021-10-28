# Game of life using Fully homomorphic encryption

A simple implementation of Conway's Game of Life built using Zama's [concrete-boolean](https://github.com/zama-ai/concrete/tree/master/concrete-boolean) library.

## Build

Just run `cargo build --release`. 

## Run

Run `./target/release/homomorphic_game_of_life` (or `./target/release/homomorphic_game_of_life.exe` on Windows). 

## What does it do exactly?

This is a very simple implementation of Conway's [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) using periodic boundary conditions, with a twist: the board is encrypted using a fully homomorphic encryption scheme ([TFHE](https://eprint.iacr.org/2018/421.pdf)) and all calculations are performed in encrypted space. In principle, calculations can thus be done by a thread or server which has no access to the state of the board. 

## Config and initial states

The file `Config.csv` is read at startup and defines the following parameters (first line of the file):

* the wait time between two updates in microseconds (no effect if much smaller than the time needed to compute one update),
* the pixel size,
* the board dimensions (if no initial state is provided; these two values are currently ignored),
* the background colour in rgb format,
* the foreground color in rgb format. 

The file `initial_state.csv` defines the initial state.
