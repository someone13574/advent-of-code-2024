use std::collections::HashSet;

fn main() {
    let input = load_input();
    let width = input.first().unwrap().len();
    let height = input.len();

    let mut sum = 0;
    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if *cell == 0 {
                sum += find_trails((row_idx, col_idx), &input, width, height);
            }
        }
    }

    println!("{sum}");
}

fn load_input() -> Vec<Vec<u8>> {
    let input = include_str!("../data/input.txt");
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim()
                .chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn find_trails(
    trail_head: (usize, usize),
    topographic_map: &[Vec<u8>],
    width: usize,
    height: usize,
) -> usize {
    let mut active_positions = HashSet::new();
    let mut next_positions = HashSet::new();

    active_positions.insert((trail_head.0, trail_head.1));

    for elevation in 1..=9 {
        // println!("searching elevation {elevation}, active: {active_positions:?}");
        for (row, col) in &active_positions {
            if *row != 0 && topographic_map[row - 1][*col] == elevation {
                next_positions.insert((row - 1, *col));
            }

            if *row != width - 1 && topographic_map[row + 1][*col] == elevation {
                next_positions.insert((row + 1, *col));
            }

            if *col != 0 && topographic_map[*row][col - 1] == elevation {
                next_positions.insert((*row, col - 1));
            }

            if *col != height - 1 && topographic_map[*row][col + 1] == elevation {
                next_positions.insert((*row, col + 1));
            }
        }

        active_positions = std::mem::take(&mut next_positions);
    }

    active_positions.len()
}
