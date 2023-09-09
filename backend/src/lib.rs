use std::{collections::VecDeque, fs::remove_dir_all};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Game {
    score: usize,
    board: Vec<Vec<usize>>,
    history: VecDeque<Self>,
}

pub enum Direction {
    U,
    R,
    L,
    D,
}

impl Game {
    /// Create a new game.
    /// `width` and `height` must be at least 3.
    /// `max_history` can be 0 to disable `undo`
    pub fn new(width: usize, height: usize, max_history: usize) -> Option<Self> {
        if width < 3 || height < 3 {
            None
        } else {
            Some(Self {
                score: 0,
                board: vec![vec![0; width]; height],
                history: VecDeque::with_capacity(max_history),
            })
        }
    }

    pub fn push(&mut self, direction: Direction) -> bool {
        unimplemented!()
    }

    fn add_to_history(&mut self) {}

    pub fn undo(&mut self) -> bool {
        unimplemented!()
    }

    fn merge_left(&mut self) {}

    fn merge_right(&mut self) {}

    fn merge_down(&mut self) {}

    fn merge_up(&mut self) {}

    fn mirror(&mut self) {
        self.board.iter_mut().for_each(|r| r.reverse())
    }

    fn transpose(&mut self) {}
}

impl Default for Game {
    fn default() -> Self {
        Self::new(4, 4, 1).unwrap()
    }
}
