use std::collections::HashSet;

pub struct MapInfo {
    width: usize,
    heigth: usize,
    tree_coords: HashSet<(usize, usize)>
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> MapInfo {
    let mut width: usize = 0;
    let mut heigth: usize = 0;
    let mut tree_coords = HashSet::new();
    input
        .lines()
        .for_each(|line| {
            width = line.len();
            line
                .chars()
                .enumerate()
                .for_each(|(i,c)| {
                    if c == '#' {
                        tree_coords.insert((i, heigth));
                    }
                });
            heigth += 1;
        });
    MapInfo { width, heigth, tree_coords }
}

fn count_trees(map_info: &MapInfo, x_step: usize, y_step: usize) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut n_trees = 0;

    while y < map_info.heigth {
        n_trees += map_info.tree_coords.contains(&(x,y)) as usize;
        x += x_step;
        if x >= map_info.width {
            x -= map_info.width
        };
        y += y_step;
    }
    n_trees
}

#[aoc(day3, part1)]
pub fn part1(input: &MapInfo) -> usize {
    count_trees(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(input: &MapInfo) -> usize {
    let step_sizes: Vec<(usize,usize)> = vec![(1,1),(3,1),(5,1),(7,1),(1,2)];
    step_sizes
        .iter()
        .map(|(x_step, y_step)| {
            count_trees(input, *x_step, *y_step)
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "..##.......\n#...#...#..\n.#....#..#.\n\
        ..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n\
        .#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        assert_eq!(part1(&input_generator(input)), 7)
    }

    #[test]
    fn test2() {
        let input = "..##.......\n#...#...#..\n.#....#..#.\n\
        ..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n\
        .#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        assert_eq!(part2(&input_generator(input)), 336)
    }
}