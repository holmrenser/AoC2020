use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| {
            l.parse().unwrap()
        }).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i64]) -> i64 {
    input
        .into_iter()
        .combinations(2)
        .fold(0, |acc, v| {
            let i1 = v[0];
            let i2 = v[1];
            if i1 + i2 == 2020 {
                i1 * i2
            } else {
                acc
            }
        })
}

#[aoc(day1, part2)]
pub fn part2(input: &[i64]) -> i64 {
    input
        .iter()
        .combinations(3)
        .fold(0, |acc, v| {
            let i1 = v[0];
            let i2 = v[1];
            let i3 = v[2];
            if i1 + i2 + i3 == 2020 {
                i1 * i2 * i3
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
        assert_eq!(input_generator("1\n2\n3"), vec![1,2,3])
    }

    #[test]
    fn test2() {
        assert_eq!(
            part1(&input_generator("1721\n979\n366\n299\n675\n1456")), 514579)
    }

    #[test]
    fn test3() {
        assert_eq!(
            part2(&input_generator("1721\n979\n366\n299\n675\n1456")),
            241861950)
    }
}