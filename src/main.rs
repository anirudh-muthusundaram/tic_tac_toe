use std::io::{self, Write};

fn main() {
    let mut board = vec![vec![' '; 3]; 3];
    let mut current_player = 'X';

    loop {
        print_board(&board);
        player_move(&mut board, current_player);
        if check_winner(&board, current_player) {
            print_board(&board);
            println!("Player {} wins!", current_player);
            break;
        }
        if board_full(&board) {
            print_board(&board);
            println!("It's a draw!");
            break;
        }
        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}

fn print_board(board: &Vec<Vec<char>>) {
    println!("Current board:");
    for row in board {
        println!(" {} | {} | {} ", row[0], row[1], row[2]);
        println!("---|---|---");
    }
}

fn player_move(board: &mut Vec<Vec<char>>, player: char) {
    loop {
        let mut input = String::new();
        print!("Player {}, enter your move (row and column): ", player);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let mut parts = input.trim().split_whitespace();
        if let (Some(row), Some(col)) = (parts.next(), parts.next()) {
            if let (Ok(row), Ok(col)) = (row.parse::<usize>(), col.parse::<usize>()) {
                if row < 3 && col < 3 && board[row][col] == ' ' {
                    board[row][col] = player;
                    break;
                }
            }
        }
        println!("Invalid move, please try again.");
    }
}

fn check_winner(board: &Vec<Vec<char>>, player: char) -> bool {
    for i in 0..3 {
        if board[i][0] == player && board[i][1] == player && board[i][2] == player {
            return true;
        }
        if board[0][i] == player && board[1][i] == player && board[2][i] == player {
            return true;
        }
    }
    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true;
    }
    if board[0][2] == player && board[1][1] == player && board[2][0] == player {
        return true;
    }
    false
}

fn board_full(board: &Vec<Vec<char>>) -> bool {
    for row in board {
        for &cell in row {
            if cell == ' ' {
                return false;
            }
        }
    }
    true
}
