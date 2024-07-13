#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
enum Player {
    X,
    O,
}

impl Player {
    /// Get the opponent for the given player
    fn opponent(self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player::X => 'X',
                Player::O => 'O',
            }
        )
    }
}

// Hack to have inquire display a nice name for the board index representing the square on the board
#[derive(Copy, Clone, Debug, Serialize)]
struct Selection {
    pub square: usize,
}

impl Selection {
    const SQUARES: [&'static str; 9] = [
        "Top Left",
        "Top Middle",
        "Top Right",
        "Middle Left",
        "Middle",
        "Middle Right",
        "Bottom Left",
        "Bottom Middle",
        "Bottom Right",
    ];

    fn new(square: usize) -> Self {
        Self { square }
    }
}

impl Display for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::SQUARES[self.square])
    }
}

/// Return the winner for a given line or None if there is no winner
fn get_line_winner(a: Option<Player>, b: Option<Player>, c: Option<Player>) -> Option<Player> {
    if a.is_some() && a == b && b == c {
        a
    } else {
        None
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum GameResult {
    Loss = -1,
    Tie = 0,
    Win = 1,
}

/// Minimax algorithm to choose the best move for the computer
fn minimax(state: &GameState) -> GameResult {
    if let Some(winner) = state.winner {
        if winner == state.computer_player {
            return GameResult::Win;
        } else {
            return GameResult::Loss;
        }
    }

    // Check tied game state
    let possible_moves = state.open_squares();
    if possible_moves.is_empty() {
        return GameResult::Tie;
    }

    if state.next_player == state.computer_player {
        // Unwrap since we already checked possible_moves.is_empty()
        possible_moves
            .iter()
            .map(|m| minimax(&state.with_move(m.square)))
            .max()
            .unwrap()
    } else {
        possible_moves
            .iter()
            .map(|m| minimax(&state.with_move(m.square)))
            .min()
            .unwrap()
    }
}

fn check_winner(board: &[Option<Player>; 9]) -> Option<Player> {
    for i in 0..3 {
        // Check rows
        if let Some(winner) = get_line_winner(board[i * 3], board[i * 3 + 1], board[i * 3 + 2]) {
            return Some(winner);
        }

        // Check columns
        if let Some(winner) = get_line_winner(board[i], board[i + 3], board[i + 6]) {
            return Some(winner);
        }
    }

    // Check diagonals
    if let Some(winner) = get_line_winner(board[0], board[4], board[8]) {
        return Some(winner);
    }

    if let Some(winner) = get_line_winner(board[2], board[4], board[6]) {
        return Some(winner);
    }

    None
}

#[derive(Clone, Debug)]
pub struct GameState {
    board: [Option<Player>; 9],
    next_player: Player,
    winner: Option<Player>,
    computer_player: Player,
}

#[derive(Debug, Deserialize)]
pub struct PartialGameState {
    board: [Option<Player>; 9],
    next_player: Player,
}

impl PartialGameState {
    fn check_winner(&self) -> Option<Player> {
        check_winner(&self.board)
    }
}

impl From<PartialGameState> for GameState {
    fn from(state: PartialGameState) -> Self {
        Self {
            board: state.board,
            next_player: state.next_player,
            winner: state.check_winner(),
            computer_player: Player::X,
        }
    }
}

impl GameState {
    #[allow(dead_code)]
    fn new(computer_player: Player) -> Self {
        Self {
            board: [None; 9],
            next_player: Player::X,
            winner: None,
            computer_player,
        }
    }

    /// Apply a move to the gamestate
    fn apply_move(&mut self, square: usize) {
        self.board[square] = Some(self.next_player);
        self.next_player = self.next_player.opponent();
        self.winner = self.check_winner();
    }

    /// Get a new `GameState` with the given move applied
    fn with_move(&self, square: usize) -> Self {
        let mut new_state = self.clone();
        new_state.apply_move(square);
        new_state
    }

    /// Get a list of the best moves
    fn get_best_computer_moves(&self, possible_moves: &[Selection]) -> Vec<Selection> {
        let mut best_so_far = GameResult::Loss;
        // The list of moves that lead to wins
        let mut winning_moves = Vec::new();

        for &m in possible_moves {
            let move_result = minimax(&self.with_move(m.square));

            if move_result > best_so_far {
                best_so_far = move_result;
                winning_moves = vec![m];
            } else if move_result == best_so_far {
                winning_moves.push(m);
            }
        }

        winning_moves
    }

    /// Randomly choose one of the best moves to avoid repetitive games
    fn get_random_computer_move(&self) -> Option<Selection> {
        let possible_moves = self.open_squares();
        if possible_moves.is_empty() {
            return None;
        }
        let best_moves = self.get_best_computer_moves(&possible_moves);
        let random_selection = rand::thread_rng().gen_range(0..best_moves.len());
        Some(best_moves[random_selection])
    }

    /// Get a list of open squares, i.e. squares that are possible options for moves
    fn open_squares(&self) -> Vec<Selection> {
        self.board
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                if s.is_none() {
                    Some(Selection::new(i))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Return the winner or None if there is no winner
    fn check_winner(&self) -> Option<Player> {
        check_winner(&self.board)
    }
}

fn get_char(square: Option<Player>) -> char {
    match square {
        Some(Player::X) => 'X',
        Some(Player::O) => 'O',
        None => '.',
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in (0..9).step_by(3) {
            s.push_str(&format!(
                " {} | {} | {} \n",
                get_char(self.board[i]),
                get_char(self.board[i + 1]),
                get_char(self.board[i + 2]),
            ));
            if i != 6 {
                s.push_str("---|---|---\n");
            }
        }
        write!(f, "{s}")
    }
}

#[debug_handler]
pub async fn calculate_move(
    State(_ctx): State<AppContext>,
    Json(partial_state): Json<PartialGameState>,
) -> Result<Response> {
    tracing::info!("{:?}", partial_state);
    let state = GameState::from(partial_state);
    let selection = state.get_random_computer_move();
    format::json(selection)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/tictactoe")
        .add("/", post(calculate_move))
}
