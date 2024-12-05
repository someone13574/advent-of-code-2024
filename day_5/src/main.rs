use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../data/input.txt");
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = load_rules(rules);
    let updates = load_updates(updates)
        .into_iter()
        .map(|update| (check_update(&update, &rules).0, update))
        .collect::<Vec<_>>();
    let good_middles = updates
        .iter()
        .filter(|(good, _)| *good)
        .map(|(_, update)| middle(update))
        .sum::<usize>();
    let corrected_bad_middles = updates
        .iter()
        .filter(|(good, _)| !good)
        .map(|(_, update)| middle(&correct_update(update, &rules)))
        .sum::<usize>();

    println!("{good_middles}, {corrected_bad_middles}");
}

fn load_rules(rules: &str) -> HashMap<usize, Vec<usize>> {
    let mut constraints: HashMap<usize, Vec<usize>> = HashMap::new();

    for (x, y) in rules
        .trim()
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
    {
        if let Some(list) = constraints.get_mut(&y) {
            list.push(x);
        } else {
            constraints.insert(y, vec![x]);
        }
    }

    constraints
}

fn load_updates(updates: &str) -> Vec<Vec<usize>> {
    updates
        .trim()
        .lines()
        .map(|line| line.split(",").map(|page| page.parse().unwrap()).collect())
        .collect()
}

fn check_update(update: &[usize], rules: &HashMap<usize, Vec<usize>>) -> (bool, usize, usize) {
    let mut printed = HashSet::new();

    for (page_idx, page) in update.iter().enumerate() {
        if let Some(required_pages) = rules.get(page) {
            for required_page in required_pages {
                let swap_idx = update.iter().position(|page| page == required_page);

                if !printed.contains(required_page) && swap_idx.is_some() {
                    return (false, page_idx, swap_idx.unwrap());
                }
            }
        }

        printed.insert(page);
    }

    (true, 0, 0)
}

fn correct_update(update: &[usize], rules: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut update = update.to_vec();

    loop {
        let (good, swap_a, swap_b) = check_update(&update, rules);
        if good {
            return update;
        }

        let (a, b) = (update[swap_a], update[swap_b]);
        update[swap_a] = b;
        update[swap_b] = a;
    }
}

fn middle(update: &[usize]) -> usize {
    assert_ne!(update.len(), 0);
    update[update.len() / 2]
}
