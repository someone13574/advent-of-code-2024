use std::collections::HashMap;

fn main() {
    let input = load_data();

    let mut p1_sum = 0;
    let mut p2_sum = 0;

    let mut lookup = HashMap::new();
    for x in input {
        p1_sum += count_stones(x, 24, &mut lookup);
        p2_sum += count_stones(x, 74, &mut lookup);
    }

    println!("Part 1: {p1_sum}");
    println!("Part 2: {p2_sum}");
}

fn load_data() -> Vec<usize> {
    let input = include_str!("../data/input.txt");
    input
        .trim()
        .split(" ")
        .into_iter()
        .map(|number| number.parse::<usize>().unwrap())
        .collect()
}

fn count_stones(
    num: usize,
    remaining: usize,
    lookup: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(stones) = lookup.get(&(num, remaining)) {
        return *stones;
    }

    if remaining == 0 {
        let stones = match num {
            0 => 1,
            x if x.ilog10() % 2 == 1 => 2,
            _ => 1,
        };

        lookup.insert((num, remaining), stones);
        stones
    } else {
        if num == 0 {
            let stones = count_stones(1, remaining - 1, lookup);
            lookup.insert((num, remaining), stones);
            return stones;
        }

        let text = num.to_string();
        if text.len() % 2 == 0 {
            let (left, right) = text.split_at(text.len() / 2);
            let left = left.parse::<usize>().unwrap();
            let right = right.parse::<usize>().unwrap();

            let stones = count_stones(left, remaining - 1, lookup)
                + count_stones(right, remaining - 1, lookup);
            lookup.insert((num, remaining), stones);
            return stones;
        }

        let stones = count_stones(num * 2024, remaining - 1, lookup);
        lookup.insert((num, remaining), stones);
        stones
    }
}
