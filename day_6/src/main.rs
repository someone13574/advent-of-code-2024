const WALL: u8 = 1;
const TRAVERSED: u8 = 2;
const UP: u8 = 4;
const DOWN: u8 = 8;
const LEFT: u8 = 16;
const RIGHT: u8 = 32;

fn main() {
    let input = include_str!("../data/input.txt");

    let (input, pos_row, pos_col) = process_input(input);
    let TraveseResult::Finish(initial_count) =
        traverse(input.clone(), pos_row, pos_col, Direction::Up)
    else {
        unreachable!()
    };

    println!("Part 1: {initial_count:?}");

    let rows = input.len();
    let cols = input[0].len();

    let mut count = 0;
    for row in 0..rows {
        for col in 0..cols {
            if row == pos_row as usize && col == pos_col as usize {
                continue;
            }

            let mut input_clone = input.clone();
            input_clone[row][col] |= WALL;

            match traverse(input_clone, pos_row, pos_col, Direction::Up) {
                TraveseResult::Loop => {
                    count += 1;
                }
                _ => {}
            }
        }
    }

    println!("Part 2: {count}");
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rot(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    pub fn next_pos(&self, row: i64, col: i64) -> (i64, i64) {
        match self {
            Self::Up => (row - 1, col),
            Self::Down => (row + 1, col),
            Self::Left => (row, col - 1),
            Self::Right => (row, col + 1),
        }
    }
}

#[derive(Debug)]
enum TraveseResult {
    Loop,
    Finish(usize),
}

fn traverse(
    mut input: Vec<Vec<u8>>,
    mut row: i64,
    mut col: i64,
    mut direction: Direction,
) -> TraveseResult {
    let mut count = 1;

    loop {
        if !cell_contains(input[row as usize][col as usize], TRAVERSED) {
            count += 1;
        } else if cell_contains(
            input[row as usize][col as usize],
            match direction {
                Direction::Up => UP,
                Direction::Down => DOWN,
                Direction::Left => LEFT,
                Direction::Right => RIGHT,
            },
        ) && count != 1
        {
            return TraveseResult::Loop;
        }

        input[row as usize][col as usize] |= TRAVERSED
            | match direction {
                Direction::Up => UP,
                Direction::Down => DOWN,
                Direction::Left => LEFT,
                Direction::Right => RIGHT,
            };

        let (next_row, next_col) = direction.next_pos(row, col);

        if next_row < 0 || next_col < 0 {
            break;
        }

        if let Some(next_row_ref) = input.get(next_row as usize) {
            if let Some(next_pos) = next_row_ref.get(next_col as usize) {
                if cell_contains(*next_pos, WALL) {
                    direction = direction.rot();
                } else {
                    row = next_row;
                    col = next_col;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    return TraveseResult::Finish(count);
}

fn process_input(input: &str) -> (Vec<Vec<u8>>, i64, i64) {
    let input = input
        .trim()
        .lines()
        .into_iter()
        .map(|line| {
            line.bytes()
                .map(|byte| match byte {
                    b'.' => 0,
                    b'#' => WALL,
                    b'^' => TRAVERSED | UP,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let (row, col) = input
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            (
                row_idx,
                row.iter().position(|cell| cell_contains(*cell, TRAVERSED)),
            )
        })
        .find(|(_, column)| column.is_some())
        .unwrap();
    (input, row as i64, col.unwrap() as i64)
}

#[inline(always)]
fn cell_contains(cell: u8, val: u8) -> bool {
    (cell & val) == val
}
