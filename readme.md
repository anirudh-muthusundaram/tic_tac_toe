# Tic Tac Toe Game in Rust
This project is a simple command-line Tic Tac Toe game implemented in Rust. Two players take turns to mark spaces on a 3x3 grid, with one player using 'X' and the other using 'O'. The game checks for a winner after each move and declares the result when a player wins or the board is full (resulting in a draw).

## Features

Command-line interface
Two-player game
Input validation
Winner detection
Draw detection

## How to Play

Clone the repository and navigate to the project directory.
Compile and run the program using the Rust compiler:
sh
Copy code
rustc main.rs
./main
Players will be prompted to enter their moves in the format of row and column indices (0-based).
The game will alternate between players until there is a winner or the board is full.