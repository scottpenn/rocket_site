use std::fmt;
use std::collections::HashMap;

pub struct TicTacToe {
    board: Vec<Vec<Move>>,
    next: Move,
    moves: Vec<TicTacToe>,
    winner: Vec<Winner>,
}

impl TicTacToe {
    fn new() -> TicTacToe {
        TicTacToe {
            board: vec![vec![Move::Empty, Move::Empty, Move::Empty]; 3],
            next: Move::X,
            moves: vec![],
            winner: vec![Winner::TBD],
        }
    }

    fn from(board: Vec<Vec<Move>>, next: Move) -> TicTacToe {
        TicTacToe {
            board,
            next,
            moves: vec![],
            winner: vec![Winner::TBD],
        }
    }

    fn generate_next(&self, board_map: &mut HashMap<Vec<Vec<Move>>, bool>) -> Vec<TicTacToe> {
        let mut boards = Vec::new();
        for x in 0..3 {
            for y in 0..3 {
                if self.board[x][y] == Move::Empty {
                    let next_move = self.make_move(x, y);
                    if board_map.contains_key(&next_move) {
                        continue;
                    }
                    board_map.insert(next_move.clone(), true);

                    let mut next_board = TicTacToe::from(next_move, Move::next(&self.next));
                    next_board.moves = next_board.generate_next(board_map);

                    boards.push(next_board);
                }
            }
        }
        boards
    }

    fn make_move(&self, x: usize, y: usize) -> Vec<Vec<Move>> {
        let mut next_board = self.board.clone();
        next_board[x][y] = self.next;
        next_board
    }

    fn determine_winner(&mut self) -> Vec<Winner> {
        if self.moves.is_empty() {
            return TicTacToe::check_win_conditions(&self.board);
        }

        for board in &mut self.moves {
            board.winner = board.determine_winner();
        }
        if self.moves.iter().any(|mv| mv.winner.contains(&Winner::TBD)) {
            return vec![Winner::TBD];
        }

        let mut winners = Vec::new();
        if self.moves.iter().all(|mv| mv.winner.contains(&Winner::X)) {
            winners.push(Winner::X);
        }
        if self.moves.iter().all(|mv| mv.winner.contains(&Winner::O)) {
            winners.push(Winner::O);
        }
        if self.moves.iter().all(|mv| mv.winner.contains(&Winner::Cat)) {
            winners.push(Winner::Cat);
        }
        if winners.is_empty() {winners.push(Winner::TBD)}
        winners
    }

    fn check_win_conditions(board: &Vec<Vec<Move>>) -> Vec<Winner> {
        let top_left     = board[0][0];
        let top          = board[0][1];
        let top_right    = board[0][2];
        let left         = board[1][0];
        let center       = board[1][1];
        let right        = board[1][2];
        let bottom_left  = board[2][0];
        let bottom       = board[2][1];
        let bottom_right = board[2][2];

        let all_top_row             = top_left == top && top_left == top_right;
        let all_middle_row          = left == center && left == right;
        let all_bottom_row          = bottom_left == bottom && bottom_left == bottom_right;
        let all_left_column         = top_left == left && top_left == bottom_left;
        let all_middle_column       = top == center && top == bottom;
        let all_right_column        = top_right == right && top_right == bottom_right;
        let all_descending_diagonal = top_left == center && top_left == bottom_right;
        let all_ascending_diagonal  = top_right == center && top_right == bottom_left;

        let mut winners = Vec::new();

        let mut push_winner = |square| {
            if square == Move::X {
                winners.push(Winner::X)
            } else {
                winners.push(Winner::O)
            }
        };

        if all_top_row || all_left_column || all_descending_diagonal {
            push_winner(top_left);
        }

        if all_middle_row || all_middle_column || all_ascending_diagonal {
            push_winner(center);
        }

        if all_bottom_row || all_right_column {
            push_winner(bottom_right);
        }
        
        if winners.is_empty() {
            winners.push(Winner::Cat);
        }
        winners
    }

    fn generate_winner_map(&self, map: &mut HashMap<String, Vec<Winner>>) {
        for board in &self.moves {
            board.generate_winner_map(map);
        }
        let mut grid = String::new();
        for x in 0..3 {
            for y in 0..3 {
                let square = match self.board[x][y] {
                    Move::X => "X",
                    Move::O => "O",
                    Move::Empty => " "
                };
                grid += square;
            }
        }
        map.insert(grid, self.winner.clone());
    }
}

impl fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for row in &self.board {
            result += &format!("{} {} {}\n", row[0], row[1], row[2]);
        }
        write!(f, "{}", result)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Move {
    X,
    O,
    Empty
}

impl Move {
    fn next(mv: &Move) -> Move {
        match mv {
            Move::X => Move::O,
            Move::O => Move::X,
            Move::Empty => Move::Empty,
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match self {
            Move::X => "X",
            Move::O => "O",
            Move::Empty => ".",
        };
        write!(f, "{}", result)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Winner {
    O,
    X,
    Cat,
    TBD,
}
