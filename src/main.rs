mod game;
use game::*;

mod ui;
use ui::*;

fn main() {
    let (initial_state, solution) = State::new_unsolved();
    init_ui(initial_state, solution).run();
}

