use std::collections::BinaryHeap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::num::ParseIntError;
use std::path::Path;
use crate::lib::Answer;

fn from_path(path: &Path) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let len = file.metadata()?.len() as usize;
    let mut contents = String::with_capacity(len);
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn from_string(s: &str) -> Result<Vec<i64>, ParseIntError> {
    let grouped = s.split("\n\n");
    let converted = grouped.map(|sub|
        // First, convert each line into a number. Then, sum them.
        // If a line can't be parsed into a number, the sum fails.
        sub.trim().split("\n")
            .map(|s| {
                s.parse::<i64>()
            })
            .fold(
                Ok(0i64),
                |acc, res| {
                    match (acc, res) {
                        (Ok(n), Ok(m)) => Ok(n + m),
                        (Ok(_), Err(e)) => Err(e),
                        (Err(e), _) => Err(e),
                    }
                })
    )
        .collect::<Result<Vec<i64>, ParseIntError>>()?;

    Ok(converted)
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

pub fn solve(path: &Path) -> Result<Answer, Box<dyn Error>> {
    let s = from_path(path)?;
    let sums = from_string(&s)?;
    let (p1, p2) = (part1(sums.iter().cloned()), part2(sums.iter().cloned()));
    Ok(Answer { part_1: p1, part_2: p2 })
}