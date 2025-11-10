use crate::{State, GRID_SIZE};

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub use cursive::{Cursive, CursiveExt};

use cursive::views::{Panel, TextView, LinearLayout, Button, OnEventView};
use cursive::view::View;
use cursive::view::{Nameable, Resizable};

use cursive::event::Event;

use cursive::utils::markup::StyledString;
use cursive::style::{Color, BaseColor};

use const_format::formatcp;

const COLORS: [Color; 8] = [
    Color::Dark(BaseColor::Black),
    Color::Dark(BaseColor::Red),
    Color::Dark(BaseColor::Green),
    Color::Dark(BaseColor::Yellow),
    Color::Dark(BaseColor::Blue),
    Color::Dark(BaseColor::Magenta),
    Color::Dark(BaseColor::Cyan),
    Color::Dark(BaseColor::White),
];

const BUTTON_WIDTH: usize = 5;
const BUTTON_HEIGHT: usize = 2;

const QUEEN: char = '*';
const MARKED: char = 'x';
const BLANK: char = '=';

const VERIFY_KEY: char = 'v';
const NEW_GAME_KEY: char = 'n';
const RESET_KEY: char = 'r';
const SOLVE_KEY: char = 's';
const CYCLE_COLOR_KEY: char = 'c';
const QUIT_KEY: char = 'q';

const GAME_HELP_MSG: &str = formatcp!("\
    Arrow keys to move. \n\
    Press <{}> to verify. \n\
    Press <{}> to reset the board. \n\
    Press <{}> for a new game. \n\
    Press <{}> to quit. \n\
    Press <ENTER> to cycle between a square: \n\
        - Being marked. (i.e. to rule out squares that can't contain a queen), \n\
        - Containing a queen. \n\
        - Being blank. \n\n\
    Queen: '{}', marked square: '{}', blank square: '{}'. \n\
    ", VERIFY_KEY, RESET_KEY, NEW_GAME_KEY, QUIT_KEY, QUEEN, MARKED, BLANK);

const SOLVER_HELP_MSG: &str = formatcp!("\
    Arrow keys to move. \n\
    Press <{}> to cycle a square's color. \n\
    Press <{}> to solve. \n\
    Press <{}> to reset (i.e. clear) the board. \n\
    Press <{}> to quit. \n\
    Press <ENTER> to cycle between a square having a queen, and being blank \n\
    Queen: '{}', blank square: '{}'. \n\n\
    Note that if there is more than one solution, only the first one found will be displayed.
    ", CYCLE_COLOR_KEY, SOLVE_KEY, RESET_KEY, QUIT_KEY, QUEEN, BLANK);

pub fn init_game_ui(state: State, solution: State, should_loop: Arc<AtomicBool>) -> Cursive {
    let mut siv = Cursive::new();

    let layout = LinearLayout::vertical()
        .child(TextView::new("Queens - Game"))
        .child(LinearLayout::horizontal()
            .child(new_game_grid(&state))
            .child(TextView::new(GAME_HELP_MSG)))
        .child(TextView::new("").with_name("msg_box"));

    siv.add_layer(layout);
    siv.set_user_data(state.clone());

    // display the validity of the current game state ("correct", "incomplete", or "incorrect") on a keypress
    siv.add_global_callback(VERIFY_KEY, move |s| {
        let current_state = s.user_data::<State>().unwrap().clone();

        let mut msg_box = s.find_name::<TextView>("msg_box").unwrap();
        if current_state == solution {
            msg_box.set_content("Correct!");
        } else if current_state.count_queens() < solution.count_queens() {
            msg_box.set_content("Incomplete");
        } else {
            msg_box.set_content("Incorrect");
        }
    });

    // on a keypress, set a flag to indicate that a new game should be made after this one, then quit
    {
        let flag = Arc::clone(&should_loop);
        siv.add_global_callback(NEW_GAME_KEY, move |s| {
            flag.store(true, Ordering::Relaxed);
            s.quit();
        });
    }

    // on a keypress, set the board back to its initial state, and update all the cells
    siv.add_global_callback(RESET_KEY, move |s| {
        s.set_user_data(state.clone());
        update_board(s);
    });

    // on a keypress, set a flag to indicate that a new game shouldn't be made after this one, then quit
    {
        let flag = Arc::clone(&should_loop);
        siv.add_global_callback(QUIT_KEY, move |s| {
            flag.store(false, Ordering::Relaxed);
            s.quit();
        });
    }

    siv
}

pub fn init_solver_ui() -> Cursive {
    let mut siv = Cursive::new();

    let state = State::new_blank();

    let layout = LinearLayout::vertical()
        .child(TextView::new("Queens - Solver"))
        .child(LinearLayout::horizontal()
            .child(new_solver_grid(&state))
            .child(TextView::new(SOLVER_HELP_MSG)))
        .child(TextView::new("").with_name("msg_box"));

    siv.add_layer(layout);
    siv.set_user_data(state);

    // on a keypress, solve the current board (if possible), and update all the cells
    siv.add_global_callback(SOLVE_KEY, move |s| {
        if let Some(new_state) = s.with_user_data(|state: &mut State| state.solutions(GRID_SIZE).next()).unwrap() {
            s.set_user_data(new_state);
            update_board(s);

            s.find_name::<TextView>("msg_box").unwrap().set_content("Solved");

        } else {
            s.find_name::<TextView>("msg_box").unwrap().set_content("Unsolvable");
        }
    });

    // clear board on keypress
    siv.add_global_callback(RESET_KEY, move |s| {
        s.set_user_data(State::new_blank());
        update_board(s);
    });

    // quit on keypress
    siv.add_global_callback(QUIT_KEY, move |s| s.quit());

    siv
}

// initializes a new grid object
fn new_game_grid(state: &State) -> impl View {
    let mut grid = LinearLayout::vertical();

    for y in 0..GRID_SIZE {
        let mut row = LinearLayout::horizontal();
        for x in 0..GRID_SIZE {
            let button = new_cell_button(state, x, y, true);
            row.add_child(button);

        }
        grid.add_child(row);
    }

    Panel::new(grid)
}

fn new_solver_grid(state: &State) -> impl View {
    let mut grid = LinearLayout::vertical();

    for y in 0..GRID_SIZE {
        let mut row = LinearLayout::horizontal();
        for x in 0..GRID_SIZE {
            let button = new_cell_button(state, x, y, false);
            let button = OnEventView::new(button)
                .on_event(Event::Char(CYCLE_COLOR_KEY),
                move |s| {
                    s.user_data::<State>().unwrap().cycle_square_group(x, y);
                    update_board(s);
                });

            row.add_child(button);
        }
        grid.add_child(row);
    }

    Panel::new(grid)
}

fn new_cell_button(initial_state: &State, x: usize, y: usize, allow_marking: bool) -> impl View {
    let label = get_button_label(initial_state, x, y);

    if initial_state.has_queen(x, y) {
        Button::new_raw(label, |_| {})
    } else {
        Button::new_raw(label,
            move |s: &mut Cursive| {
                s.with_user_data(|state: &mut State| {
                    if state.has_queen(x, y) {
                        state.unplace_queen(x, y);
                    } else if state.is_marked(x, y) {
                        state.unmark(x, y);
                        state.place_queen(x, y);
                    } else { // blank
                        if allow_marking {
                            state.mark(x, y);
                        } else {
                            state.place_queen(x, y);
                        }
                    }
                });

                update_button(s, x, y);
            })
    }.with_name(format!("cell_{}_{}", x, y))
        .fixed_width(BUTTON_WIDTH)
        .fixed_height(BUTTON_HEIGHT)
}

fn update_board(siv: &mut Cursive) {
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            update_button(siv, x, y);
        }
    }
}

fn update_button(siv: &mut Cursive, x: usize, y: usize) {
    if let Some(mut btn) = siv.find_name::<Button>(&format!("cell_{}_{}", x, y)) {
        let state = siv.user_data().unwrap();
        btn.set_label_raw(get_button_label(state, x, y));
    }
}

fn get_button_label(state: &State, x: usize, y: usize) -> StyledString {
    if state.get_cell_group(x, y) >= COLORS.len() {
        panic!("\
            Boards needing more than {} colors are not supported.\n\
            To fix this, manually add more colors to the 'COLORS' constant defined near the top of src/ui.rs.",
            COLORS.len()
        );
    }

    StyledString::styled({
        if state.has_queen(x, y) {
            QUEEN
        } else if state.is_marked(x, y) {
            MARKED
        } else {
            BLANK
        }
    }, COLORS[state.get_cell_group(x, y)])
}

