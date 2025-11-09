use crate::*;

pub use cursive::{Cursive, CursiveExt};

use cursive::views::{LinearLayout, Button};
use cursive::view::View;
use cursive::view::{Nameable, Resizable};

use cursive::utils::markup::StyledString;
use cursive::style::{Color, BaseColor};

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

pub fn init_ui(state: State) -> Cursive {
    let mut siv = Cursive::new();

    let mut layout = LinearLayout::vertical();

    for y in 0..GRID_SIZE {
        let mut row = LinearLayout::horizontal();
        for x in 0..GRID_SIZE {
            row.add_child(new_cell_button(&state, x, y));
        }
        layout.add_child(row);
    }

    siv.add_layer(layout);
    siv.set_user_data(state);

    siv
}

fn new_cell_button(state: &State, x: usize, y: usize) -> impl View {
    let label = get_button_label(state, x, y);
    Button::new_raw(label,
        move |s| {
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

            let state = s.user_data::<State>().unwrap().clone();
            update_button(&state, s, x, y);
        })
        .with_name(format!("cell_{}_{}", x, y))
        .fixed_width(BUTTON_WIDTH)
}

fn update_button(state: &State, siv: &mut Cursive, x: usize, y: usize) {
    if let Some(mut btn) = siv.find_name::<Button>(&format!("cell_{}_{}", x, y)) {
        btn.set_label_raw(get_button_label(state, x, y));
    }
}

fn get_button_label(state: &State, x: usize, y: usize) -> StyledString {
    StyledString::styled({
        if state.has_queen(x, y) {
            "*"
        } else if state.is_marked(x, y) {
            "x"
        } else {
            "="
        }
    }, COLORS[state.get_cell_group(x, y)])
}

