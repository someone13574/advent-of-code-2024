use regex::Regex;

fn main() {
    let input = include_str!("../data/input.txt");

    let mut sum = 0;
    let mut active_sum = 0;
    let mut active = true;

    for captures in Regex::new(r#"(?:mul\((\d{1,3}),(\d{1,3})\))|(do(?:n't)?\(\))"#)
        .unwrap()
        .captures_iter(input)
    {
        if let Some(cmd) = captures.get(3) {
            active = cmd.as_str() == "do()";
        } else {
            let a = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let b = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let product = a * b;

            sum += product;
            if active {
                active_sum += product;
            }
        }
    }

    println!("All:         {sum}");
    println!("Only active: {active_sum}");
}
