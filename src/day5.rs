use itertools::Itertools;

#[derive(Debug)]
pub struct Seat {
    boardingpass: Vec<char>,
    row: usize,
    column: usize,
    id: usize
}

impl Seat {
    fn from_boardingpass(input: &str) -> Self {
        let boardingpass: Vec<char> = input.chars().collect();
        let (row, _): (usize , usize) = boardingpass[..7]
            .iter()
            .fold((0,127), |(min, max), code| {
                let split = max - ((max - min) / 2);
                match code {
                    'F' => (min, split),
                    'B' => (split, max),
                    _ => panic!("Unknown code")
                }
            });
        let (column, _): (usize, usize) = boardingpass[7..]
            .iter()
            .fold((0,7), |(min, max), code| {
                let split = max - ((max - min) / 2);
                match code {
                    'L' => (min, split),
                    'R' => (split, max),
                    _ => panic!("Unknown code")
                }
            });
        let id = (row * 8) + column;
        Self { boardingpass, row, column, id }
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Seat> {
    input
        .split("\n")
        .map(|line| Seat::from_boardingpass(&line))
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &Vec<Seat>) -> usize {
    input
        .iter()
        .map(|seat| seat.id)
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &Vec<Seat>) -> usize {
    input
        .iter()
        .map(|seat| seat.id)
        .sorted()
        .collect::<Vec<usize>>()
        .windows(2)
        .fold(0, |acc, window| {
            let id1 = window[0];
            let id2 = window[1];
            if id2 - id1 == 2 {
                id2 - 1
            } else {
                acc
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL";
        assert_eq!(part1(&input_generator(input)), 820);
    }
}