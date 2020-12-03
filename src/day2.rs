use itertools::Itertools;

pub struct Policy {
    n1: usize,
    n2: usize,
    letter: char,
    password: Vec<char>
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Policy> {
    input
        .lines()
        .map(|line| {
            let (min_max, letter, password) = line
                .split(' ')
                .collect_tuple()
                .unwrap();
            let (min, max) = min_max
                .split('-')
                .collect_tuple()
                .unwrap();
            Policy { 
                n1: min.parse().unwrap(),
                n2: max.parse().unwrap(),
                letter: letter[0..1].parse().unwrap(),
                password: password.chars().collect()
            }
        }).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Policy]) -> usize {
    input
        .iter()
        .filter(|policy| {
            let letter_count = policy
                .password
                .iter()
                .filter(|p| p == &&policy.letter)
                .count();
            letter_count >= policy.n1 && letter_count <= policy.n2
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Policy]) -> usize {
    input
        .iter()
        .filter(|policy| {
            (policy.password[policy.n1 - 1] == policy.letter)
            ^ (policy.password[policy.n2 - 1] == policy.letter)
        })
        .count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(part1(&input_generator(input)), 2)
    }

    #[test]
    fn test2() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(part2(&input_generator(input)), 1)
    }
}