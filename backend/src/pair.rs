#[cfg(test)]
use std::fmt::Debug;

#[cfg(target_family = "wasm")]
use serde::{Serialize, Serializer};

type Coord = (usize, usize);

#[derive(Copy, Clone, Default)]
#[cfg_attr(test, derive(Eq))]
pub struct Pair {
    pub(crate) first: Option<Coord>,
    pub(crate) second: Option<Coord>,
}

impl Pair {
    pub(crate) fn push(&mut self, coord: Coord) {
        if self.first.is_none() {
            self.first = Some(coord);
        } else {
            self.second = Some(coord);
        }
    }

    pub(crate) fn pop(&mut self) -> Option<Coord> {
        if self.second.is_some() {
            self.second.take()
        } else {
            self.first.take()
        }
    }

    pub(crate) fn clear(&mut self) {
        self.second.take();
        self.first.take();
    }

    pub(crate) fn len(&self) -> usize {
        match (self.first.is_some(), self.second.is_some()) {
            (true, true) => 2,
            (true, false) => 1,
            (false, false) => 0,
            _ => {
                unreachable!()
            }
        }
    }
}

impl From<()> for Pair {
    fn from(_: ()) -> Self {
        Self {
            first: None,
            second: None,
        }
    }
}

impl From<Coord> for Pair {
    fn from(coord: Coord) -> Self {
        Self {
            first: Some(coord),
            second: None,
        }
    }
}

impl From<(Coord, Coord)> for Pair {
    fn from((first, second): (Coord, Coord)) -> Self {
        Self {
            first: Some(first),
            second: Some(second),
        }
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second
            || self.first == other.second && self.second == other.first
    }
}

#[cfg(test)]
impl Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.first, self.second) {
            (Some(first), Some(second)) => write!(f, "({:?}, {:?})", first, second),
            (Some(first), None) => write!(f, "{:?}", first),
            _ => {
                write!(f, "()")
            }
        }
    }
}

#[cfg(target_family = "wasm")]
impl Serialize for Pair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut array = vec![];
        if let Some(value) = self.first {
            array.push(value);
        }
        if let Some(value) = self.second {
            array.push(value);
        }
        array.serialize(serializer)
    }
}
