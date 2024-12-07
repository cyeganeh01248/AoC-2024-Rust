use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::{FxHashSet, FxHasher};

use crate::parsers::v_grid_no_whitespace;
#[aoc_generator(day6)]
fn parse(input: &str) -> (Vec<Vec<char>>, (isize, isize), u8) {
    let mut guard_r = 0isize;
    let mut guard_c = 0isize;
    let mut dir = 0u8;

    let mut grid = v_grid_no_whitespace(input);
    for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            match cell {
                '^' => {
                    guard_r = r as isize;
                    guard_c = c as isize;
                    dir = 0;
                }
                '>' => {
                    guard_r = r as isize;
                    guard_c = c as isize;
                    dir = 1;
                }
                'v' => {
                    guard_r = r as isize;
                    guard_c = c as isize;
                    dir = 2;
                }
                '<' => {
                    guard_r = r as isize;
                    guard_c = c as isize;
                    dir = 3;
                }
                _ => {}
            }
        }
    }
    grid[guard_r as usize][guard_c as usize] = '.';
    (grid, (guard_r, guard_c), dir)
}

#[aoc(day6, part1)]
fn part1((grid, (guard_r, guard_c), dir): &(Vec<Vec<char>>, (isize, isize), u8)) -> usize {
    let grid = grid.clone();

    let (raw_results, _) = traverse_grid(&grid, (*guard_r, *guard_c), *dir);

    let mut unique_results = HashSet::new();
    for (dir, r, c) in raw_results {
        unique_results.insert((r, c));
    }
    unique_results.len()
}

#[aoc(day6, part2)]
fn part2((grid, (guard_r, guard_c), dir): &(Vec<Vec<char>>, (isize, isize), u8)) -> usize {
    todo!()
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum TraverseResult {
    Edge,
    Loop,
}

fn traverse_grid(
    grid: &Vec<Vec<char>>,
    (mut guard_r, mut guard_c): (isize, isize),
    mut dir: u8,
) -> (FxHashSet<(u8, isize, isize)>, TraverseResult) {
    let mut visited = FxHashSet::default();
    visited.reserve(grid.len() * grid[0].len());
    visited.insert((dir, guard_r, guard_c));
    loop {
        let (dr, dc) = match dir {
            0 => (-1, 0),
            1 => (0, 1),
            2 => (1, 0),
            3 => (0, -1),
            _ => unreachable!(),
        };
        if guard_r + dr < 0
            || guard_r + dr >= grid.len() as isize
            || guard_c + dc < 0
            || guard_c + dc >= grid[0].len() as isize
        {
            return (visited, TraverseResult::Edge);
        } else if visited.contains(&(dir, guard_r + dr, guard_c + dc)) {
            return (visited, TraverseResult::Loop);
        } else {
            if grid[(guard_r + dr) as usize][(guard_c + dc) as usize] == '#' {
                dir = (dir + 1) % 4;
            } else {
                guard_r += dr;
                guard_c += dc;
                if visited.contains(&(dir, guard_r, guard_c)) {
                    panic!("How did we get here?");
                }
                visited.insert((dir, guard_r, guard_c));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            )),
            41
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            )),
            6
        );
    }
}