use crate::Game;

#[test]
fn new() {
    let mut max_history = 0;
    for w in 0..3 {
        for h in 0..3 {
            assert!(Game::new(w, h, max_history).is_none());
            max_history += 1;
        }
    }
    Game::default();
}
