mod game;
use game::*;

mod ui;
use ui::*;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    let should_loop = Arc::new(AtomicBool::new(true));
    
    while should_loop.load(Ordering::Relaxed) {
        let (initial_state, solution) = State::new_unsolved();
        init_ui(initial_state, solution, Arc::clone(&should_loop)).run();
    }
}

