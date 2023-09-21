use backend::{Direction, Game};
use cursive::{
    event::{Event, EventResult, Key},
    theme::{Color, ColorStyle, PaletteColor},
    traits::Resizable,
    view::SizeConstraint,
    views::Dialog,
    Printer, Vec2, View,
};
use lazy_static::lazy_static;

const HISTORY_WIDTH: usize = 13;
const SCORE_WIDTH: usize = 11;
const INTER_SPACE: usize = 2;
const OUTER_SPACE: usize = 1;

lazy_static! {
    static ref COLORS: [ColorStyle; 13] = [
        ColorStyle::new(PaletteColor::Background, Color::Rgb(204, 192, 179)),
        ColorStyle::new(PaletteColor::Background, Color::Rgb(238, 228, 218)),
        ColorStyle::new(PaletteColor::Background, Color::Rgb(237, 224, 200)),
        ColorStyle::new(PaletteColor::Background, Color::Rgb(242, 177, 121)),
        ColorStyle::new(PaletteColor::Background, Color::Rgb(245, 149, 99)),
        ColorStyle::new(PaletteColor::Background, Color::Rgb(246, 124, 95)),
        ColorStyle::new(PaletteColor::Background, Color::Rgb(246, 94, 59)),
        ColorStyle::new(PaletteColor::Background, Color::Rgb(237, 207, 114)),
        ColorStyle::new(PaletteColor::Background, Color::Rgb(237, 204, 97)),
        ColorStyle::new(PaletteColor::Background, Color::Rgb(237, 200, 80)),
        ColorStyle::new(PaletteColor::Background, Color::Rgb(237, 197, 63)),
        ColorStyle::new(PaletteColor::Background, Color::Rgb(237, 194, 46)),
        ColorStyle::new(PaletteColor::Background, Color::Rgb(0, 0, 0)),
    ];
}

pub(crate) struct Container {
    game: Game,
    width: usize,
    height: usize,
}

impl Container {
    pub(crate) fn new() -> Container {
        let game = Game::default();
        let width = game.width();
        let height = game.height();
        Container {
            game,
            width: width * 9 + width + 1,
            height: height * 5 + height + 1,
        }
    }

    fn draw_grid(&self, printer: &Printer) {
        let (width, height) = (self.game.width(), self.game.height());
        for i in 0..height {
            for j in 0..width {
                printer.print_box((j * 11 - j, i * 7 - i), (11, 7), false);
            }
        }
        for i in 1..height {
            for j in 1..width {
                printer.print((j * 11 - j, i * 7 - i), "\u{253c}");
            }
        }
        for j in 1..width {
            printer.print((j * 11 - j, 0), "\u{252c}");
            printer.print((j * 11 - j, height * 7 - height), "\u{2534}");
        }
        for i in 1..height {
            printer.print((0, i * 7 - i), "\u{251c}");
            printer.print((width * 11 - width, i * 7 - i), "\u{2524}");
        }
    }

    fn draw_cell(&self, i: usize, j: usize, printer: &Printer) {
        let value = self.game.get(i, j);
        let color = COLORS[value.checked_ilog2().unwrap_or(0) as usize];
        for line in 0..5 {
            let coord = (j * 10 + 1, i * 6 + line + 1);
            if value == 0 {
                printer.print(coord, "         ");
            } else {
                printer.with_color(color, |printer| {
                    printer.print(
                        coord,
                        &if line == 2 && value != 0 {
                            format!("{:^9}", self.game.get(i, j))
                        } else {
                            " ".repeat(9)
                        },
                    );
                });
            }
        }
    }

    fn draw_board(&self, printer: &Printer) {
        self.draw_grid(printer);
        for i in 0..self.game.height() {
            for j in 0..self.game.width() {
                self.draw_cell(i, j, printer);
            }
        }
    }

    fn draw_history(&self, printer: &Printer) {
        let history = self
            .game
            .history()
            .iter()
            .map(|h| format!("{:?}", h))
            .collect::<Vec<_>>()
            .join("\n");
        let mut dialog = Dialog::text(if history.is_empty() {
            "no history"
        } else {
            &history
        })
        .title("History")
        .resized(
            SizeConstraint::Fixed(HISTORY_WIDTH),
            SizeConstraint::AtMost(7),
        );
        dialog.layout(printer.offset);
        dialog.draw(printer);
    }

    fn draw_score(&self, printer: &Printer) {
        let mut dialog = Dialog::text(self.game.score().to_string())
            .title("Score")
            .fixed_size((SCORE_WIDTH, 3));
        dialog.layout(printer.offset);
        dialog.draw(printer);
    }
}

impl View for Container {
    fn draw(&self, printer: &Printer) {
        let (x, y) = (OUTER_SPACE, OUTER_SPACE);
        let board_padding = Vec2::new(x, y);
        let history_padding = Vec2::new(board_padding.x + self.width + INTER_SPACE, y);
        let score_padding = Vec2::new(history_padding.x + HISTORY_WIDTH + INTER_SPACE, y);

        let board_printer = printer.offset(board_padding);
        let history_printer = printer.offset(history_padding);
        let score_printer = printer.offset(score_padding);

        Container::draw_board(self, &board_printer);
        Container::draw_history(self, &history_printer);
        Container::draw_score(self, &score_printer);
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        (
            OUTER_SPACE
                + self.width
                + INTER_SPACE
                + HISTORY_WIDTH
                + INTER_SPACE
                + SCORE_WIDTH
                + OUTER_SPACE,
            (OUTER_SPACE + self.height + OUTER_SPACE).max(10),
        )
            .into()
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char('w') | Event::Key(Key::Up) => self.game.push(Direction::U),
            Event::Char('a') | Event::Key(Key::Left) => self.game.push(Direction::L),
            Event::Char('s') | Event::Key(Key::Down) => self.game.push(Direction::D),
            Event::Char('d') | Event::Key(Key::Right) => self.game.push(Direction::R),
            Event::Char('n') => {
                self.game = Game::default();
                false
            }
            Event::Char('z') => self.game.undo(),
            _ => false,
        };
        EventResult::Ignored
    }
}
