use text_io::scan;

mod game;
use game::*;

fn main() {
    let (mut current_state, solution) = State::new_unsolved();
    solution.print(); // print solution for debug purposes

    while current_state != solution {
        print!("\n\n");

        current_state.print();
        println!("Enter coords to place queen: ");

        let x: usize;
        let y: usize;
        scan!("{} {}", x, y);

        if !current_state.can_add_queen(x, y) {
            println!("Invalid position for queen");
        } else {
            current_state.place_queen(x, y);
            println!("Could be correct");
        }
    }

    println!("All correct, well done!");
    current_state.print();
}

