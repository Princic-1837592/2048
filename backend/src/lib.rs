use std::{collections::VecDeque, mem::swap};

use rand::{rngs::StdRng, RngCore, SeedableRng};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Game {
    score: usize,
    board: Vec<Vec<usize>>,
    transpose: Vec<Vec<usize>>,
    history: VecDeque<Self>,
    rng: StdRng,
}

pub enum Direction {
    U,
    R,
    L,
    D,
}

#[derive(Copy, Clone, Debug)]
struct Coord {
    i: usize,
    j: usize,
}

#[derive(Copy, Clone, Debug)]
struct Spawn {
    value: usize,
    coord: Coord,
}

impl Game {
    /// Create a new game.
    /// `width` and `height` must be at least 3 and at most 10.
    /// `max_history` can be 0 to disable `undo`
    pub fn new(height: usize, width: usize, max_history: usize) -> Option<Self> {
        Self::from_seed(height, width, max_history, rand::thread_rng().next_u64())
    }

    fn from_seed(height: usize, width: usize, max_history: usize, seed: u64) -> Option<Self> {
        if !(3..=10).contains(&width) || !(3..=10).contains(&height) {
            None
        } else {
            let mut result = Self {
                score: 0,
                board: vec![vec![0; width]; height],
                transpose: vec![vec![0; height]; width],
                history: VecDeque::with_capacity(max_history),
                rng: StdRng::seed_from_u64(seed),
            };
            result.spawn();
            result.spawn();
            Some(result)
        }
    }

    pub fn push(&mut self, direction: Direction) -> bool {
        let before = self.clone();
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
        self.spawn();
        if moved {
            self.add_to_history(before);
        }
        moved
    }

    fn move_left(&mut self) -> bool {
        let mut moved = false;
        for row in self.board.iter_mut() {
            let mut last_occupied = usize::MAX;
            for j in 0..row.len() {
                if row[j] != 0 {
                    last_occupied = last_occupied.wrapping_add(1);
                    row.swap(last_occupied, j);
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
                    row[j + 1] = 0;
                    result = true;
                }
            }
        }
        result
    }

    fn add_to_history(&mut self, state: Self) {
        if self.history.len() == self.history.capacity() {
            self.history.pop_back();
        }
        self.history.push_front(state);
    }

    pub fn undo(&mut self) -> bool {
        unimplemented!()
    }

    fn reverse(&mut self) {
        self.board.iter_mut().for_each(|r| r.reverse())
    }

    fn transpose(&mut self) {
        for i in 0..self.board.len() {
            #[allow(clippy::needless_range_loop)]
            for j in 0..self.board[0].len() {
                self.transpose[j][i] = self.board[i][j];
            }
        }
        swap(&mut self.board, &mut self.transpose);
    }

    fn spawn(&mut self) -> Spawn {
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
        // 10% probability of spawning 4
        let value = if self.rng.next_u32() % 10 == 0 { 4 } else { 2 };
        self.board[i][j] = value;
        self.score += value;
        Spawn {
            value,
            coord: Coord { i, j },
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new(4, 4, 1).unwrap()
    }
}
