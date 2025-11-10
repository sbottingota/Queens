mod game;
use game::*;

mod ui;
use ui::*;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() == 1 {
        run_game(); // run game by default

    } else if args.len() == 2 {
        match args[1].as_str() {
            "game" => run_game(),
            "solver" => run_solver(),
            _ => print_usage_msg(),
        };

    } else { // too many arguments
        print_usage_msg();
    }
}

fn run_game() {
    let should_loop = Arc::new(AtomicBool::new(true));
    
    while should_loop.load(Ordering::Relaxed) {
        let (initial_state, solution) = State::new_unsolved();
        init_game_ui(initial_state, solution, Arc::clone(&should_loop)).run();
    }
}

fn run_solver() {
    init_solver_ui().run();
}

fn print_usage_msg() {
    println!("Usage: cargo run | cargo run [game | solver]");
}

