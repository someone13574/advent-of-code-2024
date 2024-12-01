use std::collections::HashMap;

fn main() {
    let input = include_str!("../data/input.txt");
    let (mut left, mut right) = create_lists(input);

    left.sort();
    right.sort();

    let distance = get_distance(&left, &right);
    let similarity = get_similarity(&left, &right);

    println!("Distance:   {distance}");
    println!("Similarity: {similarity}");
}

fn create_lists(text: &str) -> (Vec<usize>, Vec<usize>) {
    text.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let pair = line.split_once("   ").unwrap();
            (
                pair.0.parse::<usize>().unwrap(),
                pair.1.parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn get_distance(left: &[usize], right: &[usize]) -> usize {
    left.iter()
        .zip(right)
        .map(|(left, &right)| left.abs_diff(right))
        .sum()
}

fn get_similarity(left: &[usize], right: &[usize]) -> usize {
    let mut right_counts = HashMap::new();
    for x in right {
        *right_counts.entry(x).or_default() += 1;
    }

    left.iter()
        .map(|&left| *right_counts.get(&left).unwrap_or(&0) * left)
        .sum()
}
