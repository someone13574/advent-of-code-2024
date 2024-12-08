use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../data/input.txt");

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut antinodes = HashSet::new();
    let mut resonant_antinodes = HashSet::new();

    let length = input.lines().count() as i32;
    let width = input.lines().next().unwrap().chars().count() as i32;

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if !char.is_ascii_alphanumeric() {
                continue;
            }

            if let Some(antennas) = antennas.get_mut(&char) {
                for (antenna_row, antenna_col) in antennas.iter() {
                    let delta_row = row as i32 - antenna_row;
                    let delta_col = col as i32 - antenna_col;

                    let mut antinode_row = *antenna_row;
                    let mut antinode_col = *antenna_col;
                    let mut i = 0;

                    while in_bounds(antinode_row, antinode_col, length, width) {
                        resonant_antinodes.insert((antinode_row, antinode_col));
                        if i == 1 {
                            antinodes.insert((antinode_row, antinode_col));
                        }

                        antinode_row -= delta_row;
                        antinode_col -= delta_col;
                        i += 1;
                    }

                    let mut antinode_row = row as i32;
                    let mut antinode_col = col as i32;
                    let mut i = 0;

                    while in_bounds(antinode_row, antinode_col, length, width) {
                        resonant_antinodes.insert((antinode_row, antinode_col));
                        if i == 1 {
                            antinodes.insert((antinode_row, antinode_col));
                        }

                        antinode_row += delta_row;
                        antinode_col += delta_col;
                        i += 1;
                    }
                }

                antennas.push((row as i32, col as i32));
            } else {
                antennas.insert(char, vec![(row as i32, col as i32)]);
            }
        }
    }

    let antinodes = antinodes
        .iter()
        .filter(|(row, col)| in_bounds(*row, *col, length, width))
        .collect::<HashSet<_>>();
    let resonant_antinodes = resonant_antinodes
        .iter()
        .filter(|(row, col)| in_bounds(*row, *col, length, width))
        .collect::<HashSet<_>>();

    println!("Part 1: {}", antinodes.len());
    println!("Part 2: {}", resonant_antinodes.len());
}

fn in_bounds(row: i32, col: i32, length: i32, width: i32) -> bool {
    row >= 0 && col >= 0 && row < length && col < width
}
