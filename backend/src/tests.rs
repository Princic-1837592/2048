use crate::{Direction, Game, PushResult};

#[test]
fn new() {
    let mut max_history = 0;
    for h in (0..3).chain(11..=20) {
        for w in (0..3).chain(11..=20) {
            assert!(Game::new(h, w, max_history).is_none());
            max_history += 1;
        }
    }
    assert!(Game::new(3, 3, max_history).is_some());
    assert!(Game::new(10, 10, max_history).is_some());
    Game::default();
}

#[test]
fn push() {
    let mut game = Game::from_seed(4, 4, 2, 10126721102020240073).unwrap();

    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 2],
            vec![0, 2, 0, 0],
        ]
    );
    assert_eq!(game.score, 0);
    assert_eq!(game.history.len(), 0);

    assert_eq!(
        game.push(Direction::L),
        Some(PushResult {
            transitions: vec![
                vec![().into(); 4],
                vec![().into(); 4],
                vec![(2, 3).into(), ().into(), ().into(), ().into()],
                vec![(3, 1).into(), ().into(), ().into(), ().into()],
            ],
            spawned_row: 2,
            spawned_col: 2,
            spawned_value: 2,
        })
    );
    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![2, 0, 2, 0],
            vec![2, 0, 0, 0],
        ]
    );
    assert_eq!(game.score, 0);
    assert_eq!(game.history.len(), 1);

    assert_eq!(
        game.push(Direction::R),
        Some(PushResult {
            transitions: vec![
                vec![().into(); 4],
                vec![().into(); 4],
                vec![().into(), ().into(), ().into(), ((2, 0), (2, 2)).into()],
                vec![().into(), ().into(), ().into(), (3, 0).into()],
            ],
            spawned_row: 3,
            spawned_col: 1,
            spawned_value: 2,
        })
    );
    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 4],
            vec![0, 2, 0, 2],
        ]
    );
    assert_eq!(game.score, 4);
    assert_eq!(game.history.len(), 2);

    assert!(game.undo());
    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![2, 0, 2, 0],
            vec![2, 0, 0, 0],
        ]
    );
    assert_eq!(game.score, 0);
    assert_eq!(game.history.len(), 1);

    assert_eq!(
        game.push(Direction::L),
        Some(PushResult {
            transitions: vec![
                vec![().into(); 4],
                vec![().into(); 4],
                vec![((2, 0), (2, 2)).into(), ().into(), ().into(), ().into()],
                vec![().into(); 4],
            ],
            spawned_row: 3,
            spawned_col: 1,
            spawned_value: 2,
        })
    );
    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![4, 0, 0, 0],
            vec![2, 2, 0, 0],
        ]
    );
    assert_eq!(game.score, 4);
    assert_eq!(game.history.len(), 2);

    assert_eq!(
        game.push(Direction::L),
        Some(PushResult {
            transitions: vec![
                vec![().into(); 4],
                vec![().into(); 4],
                vec![().into(); 4],
                vec![((3, 0), (3, 1)).into(), ().into(), ().into(), ().into(),],
            ],
            spawned_row: 0,
            spawned_col: 0,
            spawned_value: 2,
        })
    );
    assert_eq!(
        game.board,
        vec![
            vec![2, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![4, 0, 0, 0],
            vec![4, 0, 0, 0],
        ]
    );
    assert_eq!(game.score, 4 + 4);
    assert_eq!(game.history.len(), 2);

    assert_eq!(
        game.push(Direction::D),
        Some(PushResult {
            transitions: vec![
                vec![().into(); 4],
                vec![().into(); 4],
                vec![(0, 0).into(), ().into(), ().into(), ().into()],
                vec![((2, 0), (3, 0)).into(), ().into(), ().into(), ().into()],
            ],
            spawned_row: 1,
            spawned_col: 0,
            spawned_value: 2,
        })
    );
    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![8, 0, 0, 0],
        ]
    );
    assert_eq!(game.score, 4 + 4 + 8);
    assert_eq!(game.history.len(), 2);

    assert_eq!(
        game.push(Direction::R),
        Some(PushResult {
            transitions: vec![
                vec![().into(); 4],
                vec![().into(), ().into(), ().into(), (1, 0).into()],
                vec![().into(), ().into(), ().into(), (2, 0).into()],
                vec![().into(), ().into(), ().into(), (3, 0).into()],
            ],
            spawned_row: 0,
            spawned_col: 2,
            spawned_value: 2,
        })
    );
    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 2, 0],
            vec![0, 0, 0, 2],
            vec![0, 0, 0, 2],
            vec![0, 0, 0, 8],
        ]
    );
    assert_eq!(game.score, 4 + 4 + 8);
    assert_eq!(game.history.len(), 2);

    assert_eq!(
        game.push(Direction::U),
        Some(PushResult {
            transitions: vec![
                vec![().into(), ().into(), ().into(), ((1, 3), (2, 3)).into()],
                vec![().into(), ().into(), ().into(), (3, 3).into()],
                vec![().into(); 4],
                vec![().into(); 4],
            ],
            spawned_row: 2,
            spawned_col: 2,
            spawned_value: 2,
        })
    );
    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 2, 4],
            vec![0, 0, 0, 8],
            vec![0, 0, 2, 0],
            vec![0, 0, 0, 0],
        ]
    );
    assert_eq!(game.score, 4 + 4 + 8 + 4);
    assert_eq!(game.history.len(), 2);

    assert!(game.undo());
    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 2, 0],
            vec![0, 0, 0, 2],
            vec![0, 0, 0, 2],
            vec![0, 0, 0, 8],
        ]
    );
    assert_eq!(game.score, 4 + 4 + 8);
    assert_eq!(game.history.len(), 1);

    assert_eq!(
        game.push(Direction::U),
        Some(PushResult {
            transitions: vec![
                vec![().into(), ().into(), ().into(), ((1, 3), (2, 3)).into()],
                vec![().into(), ().into(), ().into(), (3, 3).into()],
                vec![().into(); 4],
                vec![().into(); 4],
            ],
            spawned_row: 2,
            spawned_col: 2,
            spawned_value: 2,
        })
    );
    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 2, 4],
            vec![0, 0, 0, 8],
            vec![0, 0, 2, 0],
            vec![0, 0, 0, 0],
        ]
    );
    assert_eq!(game.score, 4 + 4 + 8 + 4);
    assert_eq!(game.history.len(), 2);

    assert!(game.undo());
    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 2, 0],
            vec![0, 0, 0, 2],
            vec![0, 0, 0, 2],
            vec![0, 0, 0, 8],
        ]
    );
    assert_eq!(game.score, 4 + 4 + 8);
    assert_eq!(game.history.len(), 1);

    assert!(game.undo());
    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![8, 0, 0, 0],
        ]
    );
    assert_eq!(game.score, 4 + 4 + 8);
    assert_eq!(game.history.len(), 0);

    assert!(!game.undo());
    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![8, 0, 0, 0],
        ]
    );
    assert_eq!(game.score, 4 + 4 + 8);
    assert_eq!(game.history.len(), 0);

    assert_eq!(
        game.push(Direction::U),
        Some(PushResult {
            transitions: vec![
                vec![((1, 0), (2, 0)).into(), ().into(), ().into(), ().into()],
                vec![(3, 0).into(), ().into(), ().into(), ().into()],
                vec![().into(); 4],
                vec![().into(); 4],
            ],
            spawned_row: 0,
            spawned_col: 2,
            spawned_value: 2,
        })
    );
    assert_eq!(
        game.board,
        vec![
            vec![4, 0, 2, 0],
            vec![8, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ]
    );
    assert_eq!(game.score, 4 + 4 + 8 + 4);
    assert_eq!(game.history.len(), 1);

    assert!(game.undo());
    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![8, 0, 0, 0],
        ]
    );
    assert_eq!(game.score, 4 + 4 + 8);
    assert_eq!(game.history.len(), 0);

    assert!(game.push(Direction::L).is_none());
    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![8, 0, 0, 0],
        ]
    );
    assert_eq!(game.score, 4 + 4 + 8);
    assert_eq!(game.history.len(), 0);

    assert!(game.push(Direction::L).is_none());
    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![8, 0, 0, 0],
        ]
    );
    assert_eq!(game.score, 4 + 4 + 8);
    assert_eq!(game.history.len(), 0);

    assert_eq!(
        game.push(Direction::D),
        Some(PushResult {
            transitions: vec![
                vec![().into(); 4],
                vec![().into(); 4],
                vec![((1, 0), (2, 0)).into(), ().into(), ().into(), ().into()],
                vec![().into(); 4],
            ],
            spawned_row: 0,
            spawned_col: 2,
            spawned_value: 2,
        })
    );
    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 2, 0],
            vec![0, 0, 0, 0],
            vec![4, 0, 0, 0],
            vec![8, 0, 0, 0],
        ]
    );
    assert_eq!(game.score, 4 + 4 + 8 + 4);
    assert_eq!(game.history.len(), 1);

    assert_eq!(
        game.push(Direction::D),
        Some(PushResult {
            transitions: vec![
                vec![().into(); 4],
                vec![().into(); 4],
                vec![().into(); 4],
                vec![().into(), ().into(), (0, 2).into(), ().into()],
            ],
            spawned_row: 2,
            spawned_col: 2,
            spawned_value: 2,
        })
    );
    assert_eq!(
        game.board,
        vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![4, 0, 2, 0],
            vec![8, 0, 2, 0],
        ]
    );
    assert_eq!(game.score, 4 + 4 + 8 + 4);
    assert_eq!(game.history.len(), 2);
}
