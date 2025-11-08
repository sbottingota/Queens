use rand::prelude::*;

const GRID_SIZE: usize = 8;

#[derive(Clone, Copy, Debug)]
struct Square {
    group: usize,
    has_queen: bool,
}

impl Square {
    // return a new square with dummy values
    fn new() -> Self {
        Self { group: 0, has_queen: false }
    }

    fn is_filled(&self) -> bool {
        self.group != 0
    }
}

#[derive(Debug)]
struct State {
    grid: [[Square; GRID_SIZE]; GRID_SIZE],
    groups: Vec<Vec<(usize, usize)>>,
}

impl State {
    // return a new, blank state, without queens
    fn new() -> Self {
        let mut rng = rand::rng();

        let mut groups: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut grid = [[Square::new(); GRID_SIZE]; GRID_SIZE];

        // randomly place the initial square for each group
        let mut group_num = 1;
        while groups.len() < GRID_SIZE {
            let x = rng.random_range(0..GRID_SIZE);
            let y = rng.random_range(0..GRID_SIZE);

            // ensure that this square hasn't already been picked
            if groups.iter().any(|group| group.contains(&(x, y))) {
                continue;
            }

            groups.push(vec![(x, y)]);

            grid[x][y].group = group_num;
            group_num += 1;
        }

        // randomly add neighbours to each group until the whole board has been filled
        while groups.iter().map(|group| group.iter().count()).sum::<usize>() < GRID_SIZE*GRID_SIZE {
            let group_idx = rng.random_range(0..groups.len());

            if let Some(squares) = groups[group_idx]
                .iter()
                .map(|&square| Self::neighbors(square))
                .map(|neighbors| neighbors.into_iter().filter(|&(x, y)| !grid[x][y].is_filled()).collect::<Vec<_>>())
                .filter(|neighbors| !neighbors.is_empty())
                .next() {

                let &(x, y) = squares.choose(&mut rng).unwrap();
                groups[group_idx].push((x, y));

                grid[x][y].group = group_idx + 1;
            }
        }

        Self { grid, groups }
    }

    fn neighbors((x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut ret = Vec::new();

        if x > 0 {
            ret.push((x - 1, y));
        }
        if y > 0 {
            ret.push((x, y - 1));
        }

        if x < GRID_SIZE - 1 {
            ret.push((x + 1, y));
        }
        if y < GRID_SIZE - 1 {
            ret.push((x, y + 1))
        }

        ret
    }

    fn print(&self) {
        for row in &self.grid {
            for &Square { group, has_queen } in row {
                print!("{}{} ", group, if has_queen { '*' } else { ' ' });
            }
            println!();
        }
    }
}

fn main() {
    let state = State::new();

    state.print();
}

