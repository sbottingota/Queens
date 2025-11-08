use std::collections::{VecDeque, HashSet};

use rand::prelude::*;

const GRID_SIZE: usize = 7;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Square {
    group: Option<usize>,
    has_queen: bool,
}

impl Square {
    // return a new square with dummy values
    fn new() -> Self {
        Self { group: None, has_queen: false }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
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
        let mut group_num = 0;
        while groups.len() < GRID_SIZE {
            let x = rng.random_range(0..GRID_SIZE);
            let y = rng.random_range(0..GRID_SIZE);

            // ensure that this square hasn't already been picked
            if groups.iter().any(|group| group.contains(&(x, y))) {
                continue;
            }

            groups.push(vec![(x, y)]);

            grid[x][y].group = Some(group_num);
            group_num += 1;
        }

        // randomly add neighbours to each group until the whole board has been filled
        while groups.iter().map(|group| group.iter().count()).sum::<usize>() < GRID_SIZE*GRID_SIZE {
            let group_idx = rng.random_range(0..groups.len());

            // filter squares in the group which border unfilled squares, and take one
            if let Some(squares) = groups[group_idx]
                .iter()
                .map(|&square| Self::neighbors(square))
                .map(|neighbors| neighbors.into_iter().filter(|&(x, y)| grid[x][y].group.is_none()).collect::<Vec<_>>())
                .filter(|neighbors| !neighbors.is_empty())
                .next() {

                let &(x, y) = squares.choose(&mut rng).unwrap();
                groups[group_idx].push((x, y));

                grid[x][y].group = Some(group_idx);
            }
        }

        Self { grid, groups }
    }

    // try to find solutions by adding the specified number of queens to the state (using a DFS)
    fn solve(&self, n_queens: usize) -> Vec<Self> {
        let total_queens = n_queens + self.count_queens();

        let mut seen = HashSet::from([self.clone()]);
        let mut next = VecDeque::from([self.clone()]);

        let mut solutions = Vec::new();

        while let Some(state) = next.pop_front() {
            for x in 0..GRID_SIZE {
                for y in 0..GRID_SIZE {
                    if state.can_add_queen(x, y) {
                        let mut new_state = state.clone();
                        new_state.grid[x][y].has_queen = true;

                        if !seen.contains(&new_state) {
                            if new_state.count_queens() == total_queens {
                                solutions.push(new_state.clone());

                            } else {
                                next.push_back(new_state.clone());
                            }

                            seen.insert(new_state);
                        }
                    }
                }
            }
        }

        solutions
    }

    fn can_add_queen(&self, x: usize, y: usize) -> bool {
        // check if the group already contains a queen
        for &(x1, y1) in &self.groups[self.grid[x][y].group.expect("Grid was not properly initialized")] {
            if self.grid[x1][y1].has_queen {
                return false;
            }
        }

        // check if the square neighbors a queen
        for (x1, y1) in Self::neighbors_incl_diagonals((x, y)) {
            if self.grid[x1][y1].has_queen {
                return false;
            }
        }

        // check horizontal and vertical lines
        for x1 in 0..GRID_SIZE {
            if self.grid[x1][y].has_queen {
                return false;
            }
        }
        for y1 in 0..GRID_SIZE {
            if self.grid[x][y1].has_queen {
                return false;
            }
        }

        true
    }

    fn count_queens(&self) -> usize {
        self.grid.iter().map(|row| row.iter().filter(|square| square.has_queen).count()).sum()
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

    fn neighbors_incl_diagonals((x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut ret = Self::neighbors((x, y));

        if x > 0 {
            if y > 0 {
                ret.push((x - 1, y - 1));
            }
            if y < GRID_SIZE - 1 {
                ret.push((x - 1, y + 1));
            }
        }
        if x < GRID_SIZE - 1 {
            if y > 0 {
                ret.push((x + 1, y - 1));
            }
            if y < GRID_SIZE - 1 {
                ret.push((x + 1, y + 1));
            }
        }

        ret
    }

    fn print(&self) {
        for row in &self.grid {
            for &Square { group, has_queen } in row {
                print!("{}{} ", group.expect("Grid was not properly initialized"), if has_queen { '*' } else { ' ' });
            }
            println!();
        }
    }
}

fn main() {
    let state = State::new();
    let solutions = state.solve(GRID_SIZE);

    if solutions.is_empty() {
        println!("No solutions found");
    } else {
        for solution in solutions {
            solution.print();
            println!();
        }
    }
}

