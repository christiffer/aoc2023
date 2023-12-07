advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut final_score = 0;
    for line in input.lines() {
        let (_, winners, hands) = into_parts(line);
        final_score += score(winners, hands);
    }
    Some(final_score)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn into_parts(input: &str) -> (u32, Vec<u32>, Vec<u32>) {
    let parts: Vec<&str> = input.split(':').collect();
    let game_number = parts
        .first()
        .unwrap()
        .strip_prefix("Card")
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap();
    let game_parts: Vec<&str> = parts.last().unwrap().split('|').collect();
    let winners: Vec<u32> = game_parts
        .first()
        .unwrap()
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let hands: Vec<u32> = game_parts
        .get(1)
        .unwrap()
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    (game_number, winners, hands)
}

fn score(winners: Vec<u32>, hand: Vec<u32>) -> u32 {
    let mut score = 0;
    for win in winners {
        if hand.contains(&win) {
            score = if score == 0 { 1 } else { score * 2 };
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_parts() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let actual = into_parts(input);
        let expected: (u32, Vec<u32>, Vec<u32>) = (
            1,
            vec![41, 48, 83, 86, 17],
            vec![83, 86, 6, 31, 17, 9, 48, 53],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_winners() {
        let winners: Vec<u32> = Vec::from(&[41, 48, 83, 86, 17]);
        let card: Vec<u32> = Vec::from(&[83, 86, 6, 31, 17, 9, 48, 53]);
        let score = score(winners, card);
        assert_eq!(score, 8)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, None);
    }
}
