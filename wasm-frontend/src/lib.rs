use std::cell::RefCell;

use backend::Game;
use serde_json::to_string;
use wasm_bindgen::prelude::wasm_bindgen;

thread_local! {
static GAME: RefCell<Game> = RefCell::new(Game::default());
}

#[wasm_bindgen(js_name = new_game)]
pub fn new_game(height: usize, width: usize, max_history: usize) -> String {
    GAME.with(|game| {
        *game.borrow_mut() = Game::new(height, width, max_history).unwrap_or_default();
        to_string(game.borrow().board()).unwrap_or_default()
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
pub fn get_score() -> u32 {
    GAME.with(|game| {
        return game.borrow().score();
    })
}
