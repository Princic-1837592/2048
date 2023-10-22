use std::cell::RefCell;

use backend::Game;
use serde_json::{to_string, Number, Value};
use wasm_bindgen::prelude::wasm_bindgen;

thread_local! {
static GAME: RefCell<Game> = RefCell::new(Game::default());
}

#[wasm_bindgen(js_name = new_game)]
pub fn new_game(height: usize, width: usize, max_history: usize, seed: String) -> String {
    GAME.with(|game| {
        *game.borrow_mut() = if let Ok(seed) = seed.parse() {
            Game::from_seed(height, width, max_history, seed)
        } else {
            Game::new(height, width, max_history)
        }
        .unwrap_or_default();
        let mut result = Value::default();
        result["board"] = game
            .borrow()
            .board()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&cell| Value::Number(Number::from(cell)))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .into();
        result["seed"] = game.borrow().seed().to_string().into();
        to_string(&result).unwrap_or_default()
    })
}

#[wasm_bindgen]
pub fn push(direction: char) -> Option<String> {
    GAME.with(|game| {
        to_string(
            &direction
                .try_into()
                .ok()
                .and_then(|direction| game.borrow_mut().push(direction)),
        )
        .ok()
    })
}

#[wasm_bindgen]
pub fn undo() -> String {
    GAME.with(|game| {
        game.borrow_mut().undo();
        to_string(game.borrow().board()).unwrap_or_default()
    })
}

#[wasm_bindgen(js_name = get_state)]
pub fn get_state() -> String {
    GAME.with(|game| to_string(game.borrow().board()).unwrap_or_default())
}

#[wasm_bindgen(js_name = get_score)]
pub fn get_score() -> u64 {
    GAME.with(|game| {
        return game.borrow().score();
    })
}
