mod game;
use game::*;

mod ui;
use ui::*;

fn main() {
    let (current_state, solution) = State::new_unsolved();

    init_ui(current_state).run();
}

