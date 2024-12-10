use std::collections::HashMap;

fn main() {
    let input = load_input();
    let width = input.first().unwrap().len();
    let height = input.len();

    let mut p1_sum = 0;
    let mut p2_sum = 0;

    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if *cell == 0 {
                let (p1, p2) = find_trails((row_idx, col_idx), &input, width, height);
                p1_sum += p1;
                p2_sum += p2;
            }
        }
    }

    println!("Part 1: {p1_sum}");
    println!("Part 2: {p2_sum}");
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
) -> (usize, usize) {
    let mut active_positions = HashMap::new();
    let mut next_positions = HashMap::new();

    active_positions.insert((trail_head.0, trail_head.1), 1);

    for elevation in 1..=9 {
        for ((row, col), paths) in &active_positions {
            if *row != 0 && topographic_map[row - 1][*col] == elevation {
                next_positions
                    .entry((row - 1, *col))
                    .and_modify(|next_paths| {
                        *next_paths += *paths;
                    })
                    .or_insert(*paths);
            }

            if *row != width - 1 && topographic_map[row + 1][*col] == elevation {
                next_positions
                    .entry((row + 1, *col))
                    .and_modify(|next_paths| {
                        *next_paths += *paths;
                    })
                    .or_insert(*paths);
            }

            if *col != 0 && topographic_map[*row][col - 1] == elevation {
                next_positions
                    .entry((*row, col - 1))
                    .and_modify(|next_paths| {
                        *next_paths += *paths;
                    })
                    .or_insert(*paths);
            }

            if *col != height - 1 && topographic_map[*row][col + 1] == elevation {
                next_positions
                    .entry((*row, col + 1))
                    .and_modify(|next_paths| {
                        *next_paths += *paths;
                    })
                    .or_insert(*paths);
            }
        }

        active_positions = std::mem::take(&mut next_positions);
    }

    (
        active_positions.len(),
        active_positions
            .iter()
            .map(|(_, paths)| paths)
            .sum::<usize>(),
    )
}
