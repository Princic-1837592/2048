use std::{collections::VecDeque, mem::swap};

use rand::{rngs::StdRng, RngCore, SeedableRng};

#[cfg(test)]
mod tests;

pub const MIN_SIZE: usize = 3;
pub const DEFAULT_SIZE: usize = 4;
pub const MAX_SIZE: usize = 10;

#[derive(Clone, Debug)]
pub struct Game {
    score: u32,
    board: Vec<Vec<u32>>,
    transpose: Vec<Vec<u32>>,
    history: VecDeque<History>,
    max_history: usize,
    rng: StdRng,
}

#[derive(Clone, Debug)]
struct History {
    score: u32,
    board: Vec<Vec<u32>>,
    rng: StdRng,
    direction: Direction,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    U,
    R,
    L,
    D,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'U' => Ok(Self::U),
            'R' => Ok(Self::R),
            'L' => Ok(Self::L),
            'D' => Ok(Self::D),
            _ => Err(()),
        }
    }
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct PushResult {
    pub movements: Vec<Vec<u8>>,
    pub spawned_row: usize,
    pub spawned_col: usize,
    pub spawned_value: u32,
}

impl Game {
    /// Create a new game with a random seed
    pub fn new(height: usize, width: usize, max_history: usize) -> Option<Self> {
        Self::from_seed(height, width, max_history, rand::thread_rng().next_u64())
    }

    /// Create a new game.
    /// `width` and `height` must be at least [`MIN_SIZE`] and at most [`MAX_SIZE`].
    ///
    /// `max_history` can be 0 to disable `undo`.
    /// History vector won't be allocated to `max_history` capacity,
    /// so it's safe to pass `usize::MAX` for a virtually infinite history
    pub fn from_seed(height: usize, width: usize, max_history: usize, seed: u64) -> Option<Self> {
        if !(MIN_SIZE..=MAX_SIZE).contains(&width) || !(MIN_SIZE..=MAX_SIZE).contains(&height) {
            None
        } else {
            let mut result = Self {
                score: 0,
                board: vec![vec![0; width]; height],
                transpose: vec![vec![0; height]; width],
                history: VecDeque::new(),
                max_history,
                rng: StdRng::seed_from_u64(seed),
            };
            result.spawn();
            result.spawn();
            Some(result)
        }
    }

    pub fn push(&mut self, direction: Direction) -> Option<PushResult> {
        let before = History::new(self, self.rng.clone(), direction);
        match direction {
            Direction::U => {
                self.reverse();
                self.transpose();
            }
            Direction::R => self.reverse(),
            Direction::L => {}
            Direction::D => {
                self.transpose();
                self.reverse();
            }
        };
        let mut moved = false;
        let (this_moved, first_move) = self.move_left();
        moved |= this_moved;
        let (this_moved, merge) = self.merge_left();
        moved |= this_moved;
        let (this_moved, second_move) = self.move_left();
        moved |= this_moved;
        let mut movements = Self::merge_movements(first_move, merge, second_move);
        match direction {
            Direction::U => {
                self.transpose();
                transpose(&mut movements);
                self.reverse();
                reverse(&mut movements);
            }
            Direction::R => {
                self.reverse();
                reverse(&mut movements);
            }
            Direction::L => {}
            Direction::D => {
                self.reverse();
                reverse(&mut movements);
                self.transpose();
                transpose(&mut movements);
            }
        };
        if moved {
            let (spawned_row, spawned_col, spawned_value) = self.spawn();
            if self.max_history > 0 {
                self.add_to_history(before);
            }
            Some(PushResult {
                movements,
                spawned_row,
                spawned_col,
                spawned_value,
            })
        } else {
            None
        }
    }

    fn move_left(&mut self) -> (bool, Vec<Vec<usize>>) {
        let mut result = vec![vec![0; self.width()]; self.height()];
        let mut moved = false;
        for (i, row) in self.board.iter_mut().enumerate() {
            let mut first_empty = row.iter().position(|&v| v == 0).unwrap_or(row.len());
            for j in first_empty + 1..row.len() {
                if row[j] != 0 {
                    row.swap(first_empty, j);
                    result[i][j] = j - first_empty;
                    first_empty += 1;
                    moved = true;
                }
            }
        }
        (moved, result)
    }

    fn merge_left(&mut self) -> (bool, Vec<Vec<usize>>) {
        let mut result = vec![vec![0; self.width()]; self.height()];
        let mut moved = false;
        for (i, row) in self.board.iter_mut().enumerate() {
            for j in 0..row.len() - 1 {
                if row[j] != 0 && row[j] == row[j + 1] {
                    row[j] *= 2;
                    self.score += row[j];
                    row[j + 1] = 0;
                    result[i][j + 1] = 1;
                    moved = true;
                }
            }
        }
        (moved, result)
    }

    fn add_to_history(&mut self, state: History) {
        if self.history.len() >= self.max_history {
            self.history.pop_back();
        }
        self.history.push_front(state);
    }

    pub fn undo(&mut self) -> bool {
        if self.history.is_empty() {
            false
        } else {
            let history = self.history.pop_front().unwrap();
            self.board = history.board;
            self.score = history.score;
            self.rng = history.rng;
            true
        }
    }

    fn reverse(&mut self) {
        reverse(&mut self.board)
    }

    fn transpose(&mut self) {
        for i in 0..self.board.len() {
            for j in 0..self.board[0].len() {
                self.transpose[j][i] = self.board[i][j];
            }
        }
        swap(&mut self.board, &mut self.transpose);
    }

    fn spawn(&mut self) -> (usize, usize, u32) {
        let (mut i, mut j) = (
            self.rng.next_u32() as usize % self.board.len(),
            self.rng.next_u32() as usize % self.board[0].len(),
        );
        while self.board[i][j] != 0 {
            (i, j) = (
                self.rng.next_u32() as usize % self.board.len(),
                self.rng.next_u32() as usize % self.board[0].len(),
            );
        }
        let value = if self.rng.next_u32() % 10 == 0 { 4 } else { 2 };
        self.board[i][j] = value;
        (i, j, value)
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn height(&self) -> usize {
        self.board.len()
    }

    pub fn width(&self) -> usize {
        self.board[0].len()
    }

    pub fn max_history(&self) -> usize {
        self.max_history
    }

    pub fn board(&self) -> &Vec<Vec<u32>> {
        &self.board
    }

    pub fn history(&self) -> Vec<Direction> {
        self.history.iter().rev().map(|h| h.direction).collect()
    }

    pub fn get(&self, i: usize, j: usize) -> u32 {
        self.board[i][j]
    }

    fn merge_movements(
        mut first_move: Vec<Vec<usize>>,
        mut merge: Vec<Vec<usize>>,
        mut second_move: Vec<Vec<usize>>,
    ) -> Vec<Vec<u8>> {
        let mut result = vec![vec![0; first_move[0].len()]; first_move.len()];
        for (i, row) in result.iter_mut().enumerate() {
            for (mut j, cell) in row.iter_mut().enumerate().rev() {
                let mut total_steps = 0;
                let steps = first_move[i][j];
                total_steps += steps;
                first_move[i][j] = 0;
                j -= steps;
                let steps = merge[i][j];
                total_steps += steps;
                merge[i][j] = 0;
                j -= steps;
                let steps = second_move[i][j];
                total_steps += steps;
                second_move[i][j] = 0;
                *cell = total_steps as u8;
            }
        }
        result
    }
}

fn transpose<T: Copy>(matrix: &mut Vec<Vec<T>>) {
    let mut result = vec![vec![matrix[0][0]; matrix.len()]; matrix[0].len()];
    for (i, row) in matrix.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            result[j][i] = *value;
        }
    }
    swap(matrix, &mut result);
}

fn reverse<T>(matrix: &mut [Vec<T>]) {
    matrix.iter_mut().for_each(|r| r.reverse())
}

impl Default for Game {
    fn default() -> Self {
        Self::new(4, 4, 1).unwrap()
    }
}

impl History {
    fn new(game: &Game, rng: StdRng, direction: Direction) -> Self {
        Self {
            score: game.score,
            board: game.board.clone(),
            rng,
            direction,
        }
    }
}
