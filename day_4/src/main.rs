fn main() {
    let input = include_str!("../data/input.txt");
    let count = count_horizontal(input, b"XMAS")
        + count_horizontal(input, b"SAMX")
        + count_down(input, b"XMAS", 0)
        + count_down(input, b"SAMX", 0)
        + count_down(input, b"XMAS", -1)
        + count_down(input, b"SAMX", -1)
        + count_down(input, b"XMAS", 1)
        + count_down(input, b"SAMX", 1);

    println!("XMAS's:  {count}");
    println!("X-MAS's: {}", count_x_mas(input));
}

fn count_horizontal(input: &str, search: &[u8]) -> usize {
    input
        .lines()
        .into_iter()
        .map(|line| {
            line.as_bytes()
                .windows(search.len())
                .filter(|window| *window == search)
                .count()
        })
        .sum()
}

fn count_down(input: &str, search: &[u8], shift: i32) -> usize {
    let mut stages = Vec::new();
    let mut count = 0;

    for line in input.lines().map(|line| line.as_bytes()) {
        if stages.is_empty() {
            stages = vec![0; line.len()];
        }

        for (idx, char) in line.iter().enumerate() {
            if search[stages[idx]] == *char {
                stages[idx] += 1;
            } else if search[0] == *char {
                stages[idx] = 1;
            } else {
                stages[idx] = 0;
            }

            if stages[idx] == 4 {
                count += 1;
                stages[idx] = 0;
            }
        }

        if shift == 1 {
            stages = [&[0], &stages[..line.len() - 1]].concat();
        } else if shift == -1 {
            stages = [&stages[1..], &[0]].concat();
        }
    }

    count
}

fn count_x_mas(input: &str) -> usize {
    let input = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let columns = input[0].len();
    let mut count = 0;

    for row in 1..columns - 1 {
        for line in 1..input.len() - 1 {
            if input[line][row] != b'A' {
                continue;
            }

            match (input[line - 1][row - 1], input[line + 1][row + 1]) {
                (b'M', b'S') => {}
                (b'S', b'M') => {}
                _ => continue,
            }

            match (input[line - 1][row + 1], input[line + 1][row - 1]) {
                (b'M', b'S') => {}
                (b'S', b'M') => {}
                _ => continue,
            }

            count += 1;
        }
    }

    count
}
