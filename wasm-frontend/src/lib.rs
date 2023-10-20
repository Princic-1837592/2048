use std::cell::RefCell;

use backend::Game;
use itertools::Itertools;
use wasm_bindgen::prelude::wasm_bindgen;

thread_local! {
    static GAME: RefCell<Game> = RefCell::new(Game::default());
}

#[wasm_bindgen(js_name = newGame)]
pub fn new_game(height: usize, width: usize, max_history: usize) -> String {
    GAME.with(|game| {
        *game.borrow_mut() = Game::new(height, width, max_history).unwrap_or_default();
    });
    to_string()
}

#[wasm_bindgen]
pub fn push(direction: char) -> String {
    GAME.with(|game| {
        if let Ok(direction) = direction.try_into() {
            game.borrow_mut().push(direction);
        }
    });
    to_string()
}

#[wasm_bindgen]
pub fn undo() -> String {
    GAME.with(|game| {
        game.borrow_mut().undo();
    });
    to_string()
}

#[wasm_bindgen(js_name = getState)]
pub fn get_state() -> String {
    to_string()
}

#[wasm_bindgen(js_name = getScore)]
pub fn get_score() -> usize {
    GAME.with(|game| {
        return game.borrow().score();
    })
}

fn to_string() -> String {
    GAME.with(|game| {
        return Itertools::intersperse(
            game.borrow().board().iter().map(|x| {
                Itertools::intersperse(x.iter().map(ToString::to_string), " ".to_string())
                    .collect::<String>()
            }),
            "\n".to_string(),
        )
        .collect();
    })
}
