use std::{collections::VecDeque, mem::swap};

use rand::{rngs::StdRng, RngCore, SeedableRng};
#[cfg(target_family = "wasm")]
use serde::Serialize;

use crate::pair::Pair;

mod pair;
#[cfg(test)]
mod tests;

pub const MIN_SIZE: usize = 3;
pub const DEFAULT_SIZE: usize = 4;
pub const MAX_SIZE: usize = 10;

#[derive(Clone, Debug)]
pub struct Game {
    score: u64,
    board: Vec<Vec<u64>>,
    transpose: Vec<Vec<u64>>,
    history: VecDeque<History>,
    max_history: usize,
    rng: StdRng,
    seed: u64,
}

#[derive(Clone, Debug)]
struct History {
    score: u64,
    board: Vec<Vec<u64>>,
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
#[cfg_attr(target_family = "wasm", derive(Serialize))]
pub struct PushResult {
    pub transitions: Vec<Vec<Pair>>,
    pub spawned_row: usize,
    pub spawned_col: usize,
    pub spawned_value: u64,
    pub new_score: u64,
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
                seed,
            };
            result.spawn();
            result.spawn();
            Some(result)
        }
    }

    pub fn push(&mut self, direction: Direction) -> Option<PushResult> {
        let before = History::new(self, self.rng.clone(), direction);
        let mut transitions = vec![vec![Pair::default(); self.width()]; self.height()];
        for (i, row) in self.board.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                if *cell != 0 {
                    transitions[i][j].push((i, j));
                }
            }
        }
        match direction {
            Direction::U => {
                self.reverse(&mut transitions);
                self.transpose(&mut transitions);
            }
            Direction::R => self.reverse(&mut transitions),
            Direction::L => {}
            Direction::D => {
                self.transpose(&mut transitions);
                self.reverse(&mut transitions);
            }
        };
        let mut moved = false;
        moved |= self.move_left(&mut transitions);
        moved |= self.merge_left(&mut transitions);
        moved |= self.move_left(&mut transitions);
        match direction {
            Direction::U => {
                self.transpose(&mut transitions);
                self.reverse(&mut transitions);
            }
            Direction::R => {
                self.reverse(&mut transitions);
            }
            Direction::L => {}
            Direction::D => {
                self.reverse(&mut transitions);
                self.transpose(&mut transitions);
            }
        };
        for (i, row) in transitions.iter_mut().enumerate() {
            for (j, pair) in row.iter_mut().enumerate() {
                if pair.len() == 1 && pair.first.unwrap() == (i, j) {
                    pair.pop();
                }
            }
        }
        if moved {
            let (spawned_row, spawned_col, spawned_value) = self.spawn();
            if self.max_history > 0 {
                self.add_to_history(before);
            }
            Some(PushResult {
                transitions,
                spawned_row,
                spawned_col,
                spawned_value,
                new_score: self.score,
            })
        } else {
            None
        }
    }

    fn move_left(&mut self, transitions: &mut [Vec<Pair>]) -> bool {
        let mut moved = false;
        for (i, row) in self.board.iter_mut().enumerate() {
            let mut first_empty = row.iter().position(|&v| v == 0).unwrap_or(row.len());
            for j in first_empty + 1..row.len() {
                if row[j] != 0 {
                    row.swap(first_empty, j);
                    while let Some(value) = transitions[i][j].pop() {
                        transitions[i][first_empty].push(value);
                    }
                    first_empty += 1;
                    moved = true;
                }
            }
        }
        moved
    }

    fn merge_left(&mut self, transitions: &mut [Vec<Pair>]) -> bool {
        let mut moved = false;
        for (i, row) in self.board.iter_mut().enumerate() {
            for j in 0..row.len() - 1 {
                if row[j] != 0 && row[j] == row[j + 1] {
                    row[j] *= 2;
                    self.score += row[j];
                    row[j + 1] = 0;
                    while let Some(value) = transitions[i][j + 1].pop() {
                        transitions[i][j].push(value);
                    }
                    moved = true;
                }
            }
        }
        moved
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

    fn reverse(&mut self, transitions: &mut [Vec<Pair>]) {
        reverse(&mut self.board);
        reverse(transitions);
        for row in transitions.iter_mut() {
            row.iter_mut().for_each(|p| {
                p.first = p.first.map(|(i, j)| (i, self.width() - 1 - j));
                p.second = p.second.map(|(i, j)| (i, self.width() - 1 - j));
            })
        }
    }

    fn transpose(&mut self, #[allow(clippy::ptr_arg)] transitions: &mut Vec<Vec<Pair>>) {
        for i in 0..self.board.len() {
            for j in 0..self.board[0].len() {
                self.transpose[j][i] = self.board[i][j];
            }
        }
        swap(&mut self.board, &mut self.transpose);
        let old = transitions.clone();
        for (i, row) in old.iter().enumerate() {
            for (j, pair) in row.iter().enumerate() {
                transitions[j][i] = Pair {
                    first: pair.first.map(|(i, j)| (j, i)),
                    second: pair.second.map(|(i, j)| (j, i)),
                }
            }
        }
    }

    fn spawn(&mut self) -> (usize, usize, u64) {
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

    pub fn score(&self) -> u64 {
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

    pub fn board(&self) -> &Vec<Vec<u64>> {
        &self.board
    }

    pub fn history(&self) -> Vec<Direction> {
        self.history.iter().rev().map(|h| h.direction).collect()
    }

    pub fn get(&self, i: usize, j: usize) -> u64 {
        self.board[i][j]
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }
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
