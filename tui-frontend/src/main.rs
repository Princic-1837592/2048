use std::{
    io,
    sync::{mpsc, mpsc::Receiver},
    thread, time,
};

use backend::Game;

fn main() {
    let mut buffer = String::new();
    println!("Insert grid height (min 3 max 10)");
    io::stdin().read_line(&mut buffer).unwrap();
    let height: usize = buffer.trim_end().parse().unwrap();

    buffer.clear();
    println!("Insert grid width (min 3 max 10)");
    io::stdin().read_line(&mut buffer).unwrap();
    let width: usize = buffer.trim_end().parse().unwrap();

    buffer.clear();
    println!("Insert max history length (0 for no history)");
    io::stdin().read_line(&mut buffer).unwrap();
    let max_history: usize = buffer.trim_end().parse().unwrap();

    buffer.clear();
    println!("Insert seed (optional, press Enter for default)");
    io::stdin().read_line(&mut buffer).unwrap();
    let mut game = match buffer.trim_end().parse::<u64>() {
        Ok(seed) => Game::from_seed(height, width, max_history, seed),
        Err(_) => Game::new(height, width, max_history),
    }
    .expect("Invalid width or height");
    let mut printer = String::with_capacity(10 + 2 * game.width() * game.height());
    print(&game, &mut printer);
    println!("{}", printer);

    let stdin_channel = spawn_stdin_channel();
    loop {
        if let Ok(key) = stdin_channel.try_recv() {
            println!("Received: {}", key)
        };
        thread::sleep(time::Duration::from_millis(100));
    }
}

fn print(game: &Game, printer: &mut String) {
    printer.clear();
    printer.push_str(&format!("\n\nSCORE: {}\n\n", game.score()));
    let max_width = game
        .board()
        .iter()
        .flatten()
        .max()
        .unwrap()
        .max(&1)
        .ilog10() as usize
        + 1;
    for row in game.board() {
        printer.push('|');
        for number in row {
            printer.push_str(&format!("{: <max_width$}|", number, max_width = max_width));
        }
        printer.push('\n');
        // printer.push(' ');
        // for _ in row {
        //     printer.push_str(&"-".repeat(max_width));
        //     printer.push('+');
        // }
        // printer.pop();
        // printer.push('\n');
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}
