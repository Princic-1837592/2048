use backend::{Direction, Game};
use cursive::{
    event::{AnyCb, Event, EventResult, Key},
    theme::{Color, ColorStyle, PaletteColor},
    traits::{Nameable, Resizable},
    view::{Selector, SizeConstraint},
    views::{Dialog, LinearLayout, SliderView, TextView},
    Printer, Vec2, View,
};
use lazy_static::lazy_static;

const HISTORY_WIDTH: usize = 13;
const HISTORY_HEIGHT: usize = 7;
const SCORE_WIDTH: usize = 11;
const SCORE_HEIGHT: usize = 3;
const INTER_SPACE: usize = 2;
const OUTER_SPACE: usize = 1;
const CELL_EXT_WIDTH: usize = 13;
const CELL_EXT_HEIGHT: usize = 7;

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
    sliders: LinearLayout,
    next_width: usize,
    next_height: usize,
    next_history: usize,
}

impl Container {
    pub(crate) fn new() -> Container {
        let game = Game::default();
        let (game_width, game_height, game_history) =
            (game.width(), game.height(), game.max_history());

        let mut sliders = LinearLayout::vertical();
        sliders.add_child(
            Dialog::around(
                SliderView::horizontal((backend::MIN_SIZE..=backend::MAX_SIZE).count())
                    .value(game_width - backend::MIN_SIZE)
                    .on_change(|s, v| {
                        let title = format!("Width: {}", v + backend::MIN_SIZE);
                        s.call_on_name("width", |view: &mut Dialog| view.set_title(title));
                        s.call_on_name("container", |view: &mut Container| {
                            view.next_width = v + backend::MIN_SIZE;
                        });
                    }),
            )
            .title(format!("Width: {}", game_width))
            .with_name("width"),
        );
        sliders.add_child(
            Dialog::around(
                SliderView::horizontal((backend::MIN_SIZE..=backend::MAX_SIZE).count())
                    .value(game_height - backend::MIN_SIZE)
                    .on_change(|s, v| {
                        let title = format!("Height: {}", v + backend::MIN_SIZE);
                        s.call_on_name("height", |view: &mut Dialog| view.set_title(title));
                        s.call_on_name("container", |view: &mut Container| {
                            view.next_height = v + backend::MIN_SIZE;
                        });
                    }),
            )
            .title(format!("Height: {}", game_height))
            .with_name("height"),
        );
        sliders.add_child(
            Dialog::around(
                SliderView::horizontal(6)
                    .value(game_history)
                    .on_change(|s, v| {
                        let title = format!("History: {}", v);
                        s.call_on_name("history", |view: &mut Dialog| view.set_title(title));
                        s.call_on_name("container", |view: &mut Container| {
                            view.next_history = v;
                        });
                    }),
            )
            .title(format!("History: {}", game_history))
            .with_name("history"),
        );
        Container {
            game,
            sliders,
            next_width: backend::DEFAULT_SIZE,
            next_height: backend::DEFAULT_SIZE,
            next_history: 1,
        }
    }

    fn draw_grid(&self, printer: &Printer) {
        let (width, height) = (self.game.width(), self.game.height());
        for i in 0..height {
            for j in 0..width {
                printer.print_box(
                    (j * CELL_EXT_WIDTH - j, i * CELL_EXT_HEIGHT - i),
                    (CELL_EXT_WIDTH, CELL_EXT_HEIGHT),
                    false,
                );
            }
        }
        for i in 1..height {
            for j in 1..width {
                printer.print(
                    (j * CELL_EXT_WIDTH - j, i * CELL_EXT_HEIGHT - i),
                    "\u{253c}",
                );
            }
        }
        for j in 1..width {
            printer.print((j * CELL_EXT_WIDTH - j, 0), "\u{252c}");
            printer.print(
                (j * CELL_EXT_WIDTH - j, height * CELL_EXT_HEIGHT - height),
                "\u{2534}",
            );
        }
        for i in 1..height {
            printer.print((0, i * CELL_EXT_HEIGHT - i), "\u{251c}");
            printer.print(
                (width * CELL_EXT_WIDTH - width, i * CELL_EXT_HEIGHT - i),
                "\u{2524}",
            );
        }
    }

    fn draw_cell(&self, i: usize, j: usize, printer: &Printer) {
        let value = self.game.get(i, j);
        let color = COLORS[value.checked_ilog2().unwrap_or(0) as usize];
        for line in 0..5 {
            let coord = (
                j * (CELL_EXT_WIDTH - 1) + 1,
                i * (CELL_EXT_HEIGHT - 1) + line + 1,
            );
            if value == 0 {
                printer.print(coord, "         ");
            } else {
                printer.with_color(color, |printer| {
                    printer.print(
                        coord,
                        &if line == 2 && value != 0 {
                            format!(
                                "{:^width$}",
                                self.game.get(i, j),
                                width = CELL_EXT_WIDTH - 2
                            )
                        } else {
                            " ".repeat(CELL_EXT_WIDTH - 2)
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
            .rev()
            .map(|h| format!("{:?}", h))
            .collect::<Vec<_>>()
            .join("\n");
        let mut dialog = Dialog::around(TextView::new(if history.is_empty() {
            "no history"
        } else {
            &history
        }))
        .title("History")
        .fixed_size((HISTORY_WIDTH, HISTORY_HEIGHT));
        dialog.layout(printer.offset);
        dialog.draw(printer);
    }

    fn draw_score(&self, printer: &Printer) {
        let mut dialog = Dialog::text(self.game.score().to_string())
            .title("Score")
            .fixed_size((SCORE_WIDTH, SCORE_HEIGHT));
        dialog.layout(printer.offset);
        dialog.draw(printer);
    }

    fn draw_sliders(&self, printer: &Printer) {
        self.sliders.draw(printer);
    }

    fn draw_instructions(&self, printer: &Printer) {
        let mut dialog =
            Dialog::text("wasd or arrow keys to move\nn to start a new game\nz to undo\nq to quit")
                .title("Instructions")
                .resized(SizeConstraint::Free, SizeConstraint::Fixed(6));
        dialog.layout(printer.offset);
        dialog.draw(printer);
    }

    fn width(&self) -> usize {
        self.game.width() * (CELL_EXT_WIDTH - 1) + 1
    }

    fn height(&self) -> usize {
        self.game.height() * (CELL_EXT_HEIGHT - 1) + 1
    }

    fn offsets(&self) -> (Vec2, Vec2, Vec2, Vec2, Vec2) {
        let (x, y) = (OUTER_SPACE, OUTER_SPACE);
        let board_offset = Vec2::new(x, y);
        let history_offset = Vec2::new(board_offset.x + self.width() + INTER_SPACE, y);
        let score_offset = Vec2::new(history_offset.x + HISTORY_WIDTH + INTER_SPACE, y);
        let sliders_offset = Vec2::new(score_offset.x, score_offset.y + SCORE_HEIGHT);
        let instructions_offset = Vec2::new(
            history_offset.x,
            sliders_offset.y + HISTORY_HEIGHT + INTER_SPACE,
        );
        (
            board_offset,
            history_offset,
            score_offset,
            sliders_offset,
            instructions_offset,
        )
    }
}

impl View for Container {
    fn draw(&self, printer: &Printer) {
        let (board_offset, history_offset, score_offset, sliders_offset, instructions_offset) =
            self.offsets();

        let board_printer = printer.offset(board_offset);
        let history_printer = printer.offset(history_offset);
        let score_printer = printer.offset(score_offset);
        let sliders_printer = printer.offset(sliders_offset);
        let instructions_printer = printer.offset(instructions_offset);

        Container::draw_board(self, &board_printer);
        Container::draw_history(self, &history_printer);
        Container::draw_score(self, &score_printer);
        Container::draw_sliders(self, &sliders_printer);
        Container::draw_instructions(self, &instructions_printer);
    }

    fn layout(&mut self, constraints: Vec2) {
        self.sliders.layout(constraints);
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        (
            OUTER_SPACE
                + self.width()
                + INTER_SPACE
                + HISTORY_WIDTH
                + INTER_SPACE
                + SCORE_WIDTH.max(self.sliders.required_size(constraint).x)
                + OUTER_SPACE,
            (OUTER_SPACE + self.height() + OUTER_SPACE).max(10),
        )
            .into()
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char('w') | Event::Key(Key::Up) => {
                self.game.push(Direction::U);
            }
            Event::Char('a') | Event::Key(Key::Left) => {
                self.game.push(Direction::L);
            }
            Event::Char('s') | Event::Key(Key::Down) => {
                self.game.push(Direction::D);
            }
            Event::Char('d') | Event::Key(Key::Right) => {
                self.game.push(Direction::R);
            }
            Event::Char('n') => {
                self.game = Game::new(self.next_width, self.next_height, self.next_history)
                    .unwrap_or_default();
            }
            Event::Char('z') => {
                self.game.undo();
            }
            event @ Event::Mouse { .. } => {
                return self.sliders.on_event(event.relativized(self.offsets().3))
            }
            _ => {}
        };
        EventResult::Ignored
    }

    fn call_on_any(&mut self, selector: &Selector, cb: AnyCb) {
        self.sliders.call_on_any(selector, cb);
    }
}
