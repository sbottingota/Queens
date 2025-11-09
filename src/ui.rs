use crate::{State, GRID_SIZE};

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub use cursive::{Cursive, CursiveExt};

use cursive::views::{Panel, TextView, LinearLayout, Button};
use cursive::view::View;
use cursive::view::{Nameable, Resizable};

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
const QUIT_KEY: char = 'q';

const HELP_MSG: &str = formatcp!("\
    Arrow keys to move. \n\
    Press <{}> to verify. \n\
    Press <{}> to reset the board. \n\
    Press <{}> for a new game. \n\
    Press <{}> to quit. \n\
    Press <ENTER> to cycle between a square: \n\
        - Being marked. (i.e. to rule out squares that can't contain a queen), \n\
        - Containing a queen. \n\
        - Being blank. \n\n\
    Queen: '{}', Marked square: '{}' \n\
    ", VERIFY_KEY, RESET_KEY, NEW_GAME_KEY, QUIT_KEY, QUEEN, MARKED);

pub fn init_ui(state: State, solution: State, should_loop: Arc<AtomicBool>) -> Cursive {
    let mut siv = Cursive::new();

    let mut grid = LinearLayout::vertical();

    for y in 0..GRID_SIZE {
        let mut row = LinearLayout::horizontal();
        for x in 0..GRID_SIZE {
            row.add_child(new_cell_button(&state, x, y));
        }
        grid.add_child(row);
    }

    let layout = LinearLayout::vertical()
        .child(TextView::new("Queens"))
        .child(LinearLayout::horizontal()
            .child(Panel::new(grid))
            .child(TextView::new(HELP_MSG)))
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

        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                update_button(s, x, y);
            }
        }
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

fn new_cell_button(initial_state: &State, x: usize, y: usize) -> impl View {
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
                    } else {
                        state.mark(x, y);
                    }
                });

                update_button(s, x, y);
            })
    }.with_name(format!("cell_{}_{}", x, y))
        .fixed_width(BUTTON_WIDTH)
        .fixed_height(BUTTON_HEIGHT)
}

fn update_button(siv: &mut Cursive, x: usize, y: usize) {
    if let Some(mut btn) = siv.find_name::<Button>(&format!("cell_{}_{}", x, y)) {
        let state = siv.user_data().unwrap();
        btn.set_label_raw(get_button_label(state, x, y));
    }
}

fn get_button_label(state: &State, x: usize, y: usize) -> StyledString {
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

