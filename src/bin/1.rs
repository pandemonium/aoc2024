use std::fs;

fn extract_columns(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|pair| (pair[0], pair[1]))
        .unzip()
}

fn run_for_input1(text: &str) -> i32 {
    let (mut p, mut q) = extract_columns(text);
    p.sort();
    q.sort();

    p.into_iter().zip(q).map(|(p, q)| (p - q).abs()).sum()
}

fn run_for_input2(text: &str) -> i32 {
    let (mut p, mut q) = extract_columns(text);
    p.sort();
    q.sort();

    p.into_iter()
        .map(|p| p * q.iter().filter(|&q| p == *q).count() as i32)
        .sum()
}

fn main() {
    let test_input = "3   4
4   3
2   5
1   3
3   9
3   3";

    let input = fs::read_to_string("input-1.txt").unwrap();

    let answer = run_for_input2(&input);

    println!("Similarity score: {}", answer);
}
