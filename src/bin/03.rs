use std::cmp::{max, min};
use std::io::BufRead;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn find_cells_to_scan(
    i: &usize,
    i_max: &usize,
    j: &usize,
    j_max: &usize,
    engine_part: &u8,
) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    if engine_part.is_ascii_punctuation() && !engine_part.eq(&b'.') {
        for ii in max(0, i - 1)..min(*i_max, i + 2) {
            for jj in max(0, j - 1)..min(*j_max, j + 2) {
                result.push((ii, jj));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan() {
        let actual = find_cells_to_scan(&5, &10, &5, &10, &b'&');
        assert_eq!(
            actual,
            Vec::from_iter([
                (4, 4),
                (4, 5),
                (4, 6),
                (5, 4),
                (5, 5),
                (5, 6),
                (6, 4),
                (6, 5),
                (6, 6),
            ])
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, None);
    }
}
