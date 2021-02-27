use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn part1<T>(input: &HashSet<T>, goal: T) -> T
  where T : Eq + std::hash::Hash + Copy + std::ops::Sub<Output=T> + std::ops::Mul<Output=T> {
    return input
        .iter()
        .map(|a| (*a, goal - *a))
        .filter(|(_, b)| input.contains(b))
        .map(|(a, b)| a * b)
        .next()
        .unwrap();
}

fn part2<T>(input: &HashSet<T>, goal: T) -> T
  where T : Eq + std::hash::Hash + Copy + std::ops::Sub<Output=T> + std::ops::Mul<Output=T> {
    return input
        .iter()
        .cartesian_product(input.iter())
        .map(|(a, b)| (*a, *b, goal - *a - *b))
        .filter(|(_, _, c)| input.contains(c))
        .map(|(a, b, c)| a * b * c)
        .next()
        .unwrap();
}

fn main() {
    println!("Advent of Code 2020 - Rust - Day 01");

    let input: HashSet<i32> = BufReader::new(File::open("input/puzzle.txt").unwrap())
        .lines()
        .map(|nr| nr.unwrap().parse().unwrap())
        .collect();

    println!("Part 1 Solution: {}", part1(&input, 2020));
    println!("Part 2 Solution: {}", part2(&input, 2020));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input: HashSet<i32> = "1721
979
366
299
675
1456"
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();

        assert_eq!(514579, part1(&input, 2020));
    }

    #[test]
    fn part2_test() {
        let input: HashSet<i32> = "1721
979
366
299
675
1456"
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();

        assert_eq!(241861950, part2(&input, 2020));
    }
}
