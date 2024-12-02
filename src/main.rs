use std::{
    io::{self, Read},
    iter,
};

#[derive(Debug)]
struct Report(Vec<i32>);

impl Report {
    fn is_safe(&self) -> bool {
        let mut data = self.0.clone();

        (0..data.len()).any(|i| {
            let x = data.remove(i);
            let s = Self::is_data_safe(&data);
            data.insert(i, x);
            s
        })
    }

    fn is_data_safe(data: &[i32]) -> bool {
        let windows = data.windows(2);

        let group_count = windows.len();

        let x = windows.clone().filter(|xs| xs[0] < xs[1]).count();
        let monotonic = x == 0 || x == group_count;
        let adjacents_normal = windows
            .clone()
            .map(|xs| (xs[0] - xs[1]).abs())
            .all(|x| x >= 1 && x <= 3);

        monotonic && adjacents_normal
    }
}

fn extract_reports(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| {
            Report(
                line.split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn run_for_input(input: &str) -> usize {
    extract_reports(input)
        .iter()
        .filter(|x| x.is_safe())
        .count()
}

fn main() {
    let mut buf = String::with_capacity(1024);
    io::stdin().read_to_string(&mut buf).unwrap();

    let answer = run_for_input(&buf);

    println!("Answer: {}", answer);
}
