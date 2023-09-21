use cursive::{traits::Nameable, view::Selector};

use crate::container::Container;

mod container;

fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    let container = Container::new().with_name("container");
    siv.add_layer(container);
    siv.focus(&Selector::Name("container")).unwrap();

    siv.run();
}
