use std::{
    io,
    io::{stdout, Result as IOResult},
    num::ParseIntError,
    time::Duration,
};

use backend::{Direction, Game};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

fn main() -> IOResult<()> {
    let mut buffer = String::new();
    println!("Insert grid height (min 3 max 10)");
    io::stdin().read_line(&mut buffer).unwrap();
    let height: usize = buffer.trim_end().parse().unwrap();

    buffer.clear();
    println!("Insert grid width (min 3 max 10)");
    io::stdin().read_line(&mut buffer).unwrap();
    let width: usize = buffer.trim_end().parse().unwrap();

    buffer.clear();
    println!("Insert max history length (press Enter for no history)");
    io::stdin().read_line(&mut buffer).unwrap();
    let max_history: usize = buffer.trim_end().parse().unwrap_or(0);

    buffer.clear();
    println!("Insert seed (optional, press Enter for random)");
    io::stdin().read_line(&mut buffer).unwrap();
    let seed: Result<u64, ParseIntError> = buffer.trim_end().parse();

    let mut game = match seed {
        Ok(seed) => Game::from_seed(height, width, max_history, seed),
        Err(_) => Game::new(height, width, max_history),
    }
    .expect("Invalid width or height");

    buffer.clear();
    buffer.reserve(10 + 2 * game.width() * game.height());
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    display(&game, &mut buffer)?;
    enable_raw_mode()?;

    loop {
        if poll(Duration::from_millis(1))? {
            if let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) = read()?
            {
                let moved = match code {
                    KeyCode::Up | KeyCode::Char('w') => game.push(Direction::U),
                    KeyCode::Left | KeyCode::Char('a') => game.push(Direction::L),
                    KeyCode::Down | KeyCode::Char('s') => game.push(Direction::D),
                    KeyCode::Right | KeyCode::Char('d') => game.push(Direction::R),
                    KeyCode::Char('z') => game.undo(),
                    KeyCode::Char('q') => break,
                    _ => false,
                };
                if moved {
                    display(&game, &mut buffer)?;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), Show, LeaveAlternateScreen)?;

    Ok(())
}

fn display(game: &Game, buffer: &mut String) -> IOResult<()> {
    buffer.clear();
    buffer.push_str(&format!(
        "WASD or arrows to move.\nZ to undo.\nQ to quit\n\nSCORE: {}\n\n",
        game.score()
    ));
    let max_width = game
        .board()
        .iter()
        .flatten()
        .max()
        .unwrap()
        .max(&16)
        .ilog10() as usize
        + 1;
    buffer.push(' ');
    for _ in 0..game.width() {
        buffer.push_str(&"-".repeat(max_width));
        buffer.push('+');
    }
    buffer.pop();
    buffer.push('\n');
    for row in game.board() {
        buffer.push('|');
        for number in row {
            buffer.push_str(&format!("{: <max_width$}|", number, max_width = max_width));
        }
        buffer.push('\n');
        buffer.push(' ');
        for _ in 0..game.width() {
            buffer.push_str(&"-".repeat(max_width));
            buffer.push('+');
        }
        buffer.pop();
        buffer.push('\n');
    }
    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0),
        Print(&buffer)
    )?;
    Ok(())
}
