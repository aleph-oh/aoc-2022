use std::collections::BinaryHeap;
use std::error::Error;
use std::path::Path;
use crate::lib::Answer;

const INP: &str = include_str!("../../day1_input.txt");

fn from_string(s: &str) -> Vec<i64> {
    let grouped = s.split("\n\n");
    let sums = grouped
        .map(|sub|
            // First, convert each line into a number. Then, sum them.
            // If a line can't be parsed into a number, the sum fails.
            sub.trim()
                .lines()
                .map(|s| {
                    s.parse::<i64>().expect(&*format!("failed to parse line as number: {}", s))
                })
                .sum()
        );
    sums.collect()
}

fn part1(it: impl Iterator<Item=i64>) -> i64 {
    it.max().expect("expected at least 1 element")
}

fn part2(it: impl Iterator<Item=i64>) -> i64 {
    let mut heap: BinaryHeap<_> = it.collect();
    assert!(heap.len() >= 3, "expected at least 3 elements");
    let mut max_3 = [0i64; 3];
    for i in 0..3 {
        max_3[i] = heap.pop().expect("expected at least 3 elements");
    }

    max_3.iter().sum()
}

pub fn solve() -> Result<Answer, Box<dyn Error>> {
    let sums = from_string(INP);
    let (p1, p2) = (part1(sums.iter().cloned()), part2(sums.iter().cloned()));
    Ok(Answer { part_1: p1, part_2: p2 })
}