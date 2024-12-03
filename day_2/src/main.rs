fn main() {
    let input = include_str!("../data/input.txt");
    let reports = load_reports(input);

    let (strict, dampened) = reports.into_iter().map(|report| report.safe()).fold(
        (0, 0),
        |(strict_count, dampened_count), (strict, dampened)| {
            (
                strict_count + strict as usize,
                dampened_count + dampened as usize,
            )
        },
    );

    println!("Strict:   {strict}");
    println!("Dampened: {dampened}");
}

#[derive(Debug)]
struct Report {
    pub levels: Vec<usize>,
}

impl Report {
    fn safe(&self) -> (bool, bool) {
        let (ascending_strict, ascending_dampened) =
            self.test(|prev, curr| curr > prev && curr - prev <= 3);
        let (descending_strict, descending_dampended) =
            self.test(|prev, curr| prev > curr && prev - curr <= 3);

        (
            ascending_strict || descending_strict,
            ascending_dampened || descending_dampended,
        )
    }

    fn test(&self, condition: fn(usize, usize) -> bool) -> (bool, bool) {
        let mut dampened = 0;
        let mut skip = false;

        for idx in 1..self.levels.len() {
            if skip {
                skip = false;
                continue;
            }

            let prev = self.levels[idx - 1];
            let curr = self.levels[idx];

            if condition(prev, curr) {
                continue;
            }

            dampened += 1;

            match (
                self.levels.get(idx + 1),
                if idx < 2 {
                    None
                } else {
                    self.levels.get(idx - 2)
                },
            ) {
                (None, _) => continue,
                (Some(&next), _) if condition(prev, next) => {
                    skip = true;
                    continue;
                }
                (Some(_), None) => continue,
                (Some(_), Some(&prev_prev)) if condition(prev_prev, curr) => continue,
                _ => return (false, false),
            }
        }

        (dampened == 0, dampened <= 1)
    }
}

fn load_reports(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|number| number.parse::<usize>().unwrap())
                .collect()
        })
        .map(|levels| Report { levels })
        .collect()
}
