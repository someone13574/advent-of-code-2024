fn main() {
    let input = include_str!("../data/input.txt");
    let input = parse_input(input);

    let mut sum = 0;
    let mut sum2 = 0;

    for (test_value, numbers) in input {
        if possibly_true_part1(test_value, &numbers) {
            sum += test_value;
        }

        if possibly_true_part2(test_value, &numbers) {
            sum2 += test_value;
        }
    }

    println!("Part 1: {sum}");
    println!("Part 2: {sum2}");
}

fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| line.split_once(":").unwrap())
        .map(|(test_value, numbers)| {
            (
                test_value.parse::<usize>().unwrap(),
                numbers
                    .trim()
                    .split(" ")
                    .map(|number| number.parse::<usize>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn possibly_true_part1(test_value: usize, numbers: &[usize]) -> bool {
    let max_x = 2_usize.pow(numbers.len() as u32 - 1);

    for x in 0..max_x {
        let mut val = numbers[0];

        for (idx, number) in numbers[1..].iter().enumerate() {
            if (x >> idx) & 0x1 == 0x1 {
                val += number;
            } else {
                val *= number;
            }
        }

        if val == test_value {
            return true;
        }
    }

    false
}

fn possibly_true_part2(test_value: usize, numbers: &[usize]) -> bool {
    let max_x = 3_usize.pow(numbers.len() as u32 - 1);

    let concat_powers = numbers[1..]
        .iter()
        .map(|number| 10_usize.pow(number.ilog10() + 1))
        .collect::<Vec<_>>();

    for x in 0..max_x {
        let mut val = numbers[0];
        let mut current_x = x;

        for (idx, number) in numbers[1..].iter().enumerate() {
            match current_x % 3 {
                0 => {
                    val += number;
                }
                1 => {
                    val *= number;
                }
                2 => {
                    val *= concat_powers[idx];
                    val += number;
                }
                _ => unreachable!(),
            };
            current_x /= 3;
        }

        if val == test_value {
            return true;
        }
    }

    false
}
