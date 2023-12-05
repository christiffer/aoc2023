advent_of_code::solution!(1);

const NUMBERS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = Some(0);
    for line in input.lines() {
        let first = line.bytes().find(|b| b.is_ascii_digit()).unwrap() - b'0';
        let last = line.bytes().rfind(|b| b.is_ascii_digit()).unwrap() - b'0';
        result = Some((first * 10 + last) as u32 + result?)
    }
    result
}

fn first_word_digit(bytes: &[u8]) -> (u8, &[u8]) {
    let mut bytes = bytes;
    let result = 'first_loop: loop {
        if bytes[0].is_ascii_digit() {
            break 'first_loop bytes[0] - b'0';
        }
        for (value, number) in NUMBERS.iter().enumerate() {
            if bytes.starts_with(number) {
                break 'first_loop (value + 1) as u8;
            }
        }
        bytes = &bytes[1..];
    };
    (result, bytes)
}

fn last_word_digit(bytes: &[u8]) -> (u8, &[u8]) {
    let mut bytes = bytes;
    let result = 'last_loop: loop {
        if bytes[bytes.len() - 1].is_ascii_digit() {
            break 'last_loop bytes[bytes.len() - 1] - b'0';
        }
        for (value, number) in NUMBERS.iter().enumerate() {
            if bytes.ends_with(number) {
                break 'last_loop (value + 1) as u8;
            }
        }
        bytes = &bytes[..bytes.len() - 1];
    };
    (result, bytes)
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let (first, bytes) = first_word_digit(bytes);
            let (last, _) = last_word_digit(bytes);
            Some(first as u32 * 10 + last as u32)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_last_word_digit() {
        let input = "zzzzeighthreezzzz";
        let (first, bytes) = first_word_digit(input.as_bytes());
        let (last, _) = last_word_digit(bytes);
        assert_eq!(first, 8_u8);
        assert_eq!(last, 3_u8);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let input = "two1nine
                        eightwothree
                        abcone2threexyz
                        xtwone3four
                        4nineeightseven2
                        zoneight234
                        7pqrstsixteen";
        let result = part_two(input);
        assert_eq!(result, Some(281));
    }
}
