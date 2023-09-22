use std::{collections::VecDeque, mem::swap};

use rand::{rngs::StdRng, RngCore, SeedableRng};

#[cfg(test)]
mod tests;

pub const MIN_SIZE: usize = 3;
pub const MAX_SIZE: usize = 10;

#[derive(Clone, Debug)]
pub struct Game {
    score: usize,
    board: Vec<Vec<usize>>,
    transpose: Vec<Vec<usize>>,
    history: VecDeque<History>,
    max_history: usize,
    rng: StdRng,
}

#[derive(Clone, Debug)]
struct History {
    score: usize,
    board: Vec<Vec<usize>>,
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

    pub fn push(&mut self, direction: Direction) -> bool {
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
        moved |= self.move_left();
        moved |= self.merge_left();
        moved |= self.move_left();
        match direction {
            Direction::U => {
                self.transpose();
                self.reverse();
            }
            Direction::R => self.reverse(),
            Direction::L => {}
            Direction::D => {
                self.reverse();
                self.transpose();
            }
        };
        if moved {
            self.spawn();
            if self.max_history > 0 {
                self.add_to_history(before);
            }
        }
        moved
    }

    fn move_left(&mut self) -> bool {
        let mut moved = false;
        for row in self.board.iter_mut() {
            let mut first_empty = row.iter().position(|&v| v == 0).unwrap_or(row.len());
            for j in first_empty + 1..row.len() {
                if row[j] != 0 {
                    row.swap(first_empty, j);
                    first_empty += 1;
                    moved = true;
                }
            }
        }
        moved
    }

    fn merge_left(&mut self) -> bool {
        let mut result = false;
        for row in self.board.iter_mut() {
            for j in 0..row.len() - 1 {
                if row[j] != 0 && row[j] == row[j + 1] {
                    row[j] *= 2;
                    self.score += row[j];
                    row[j + 1] = 0;
                    result = true;
                }
            }
        }
        result
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
        self.board.iter_mut().for_each(|r| r.reverse())
    }

    fn transpose(&mut self) {
        for i in 0..self.board.len() {
            for j in 0..self.board[0].len() {
                self.transpose[j][i] = self.board[i][j];
            }
        }
        swap(&mut self.board, &mut self.transpose);
    }

    fn spawn(&mut self) {
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
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn height(&self) -> usize {
        self.board.len()
    }

    pub fn width(&self) -> usize {
        self.board[0].len()
    }

    pub fn board(&self) -> &Vec<Vec<usize>> {
        &self.board
    }

    pub fn history(&self) -> Vec<Direction> {
        self.history.iter().rev().map(|h| h.direction).collect()
    }

    pub fn get(&self, i: usize, j: usize) -> usize {
        self.board[i][j]
    }
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
