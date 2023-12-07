advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut final_score = 0;
    for line in input.lines() {
        let (winners, hands) = into_parts(line);
        final_score += score(winners, hands);
    }
    Some(final_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let winner_count = decide_winner_count(input);

    let mut number_of_cards: Vec<u32> = vec![1; winner_count.len()];
    let mut scoring_cards = 1;
    for (index, count) in winner_count.iter().enumerate() {
        for i in 1..*count + 1 {
            let target_card = index + i as usize;
            let current_num_of_cards = *number_of_cards.get(target_card).unwrap();
            let new_card_total = current_num_of_cards + scoring_cards;
            number_of_cards.remove(target_card);
            number_of_cards.insert(target_card, new_card_total);
        }
        scoring_cards = number_of_cards
            .get(index + 1 as usize)
            .unwrap_or(&1_u32)
            .to_owned();
    }

    Some(number_of_cards.iter().sum())
}

fn decide_winner_count(input: &str) -> Vec<u32> {
    let mut winner_counts: Vec<u32> = Vec::new();
    for line in input.lines() {
        let (winners, hand) = into_parts(line);
        let mut count = 0;
        for win in winners {
            if hand.contains(&win) {
                count += 1;
            }
        }
        winner_counts.push(count)
    }
    winner_counts
}

fn into_parts(input: &str) -> (Vec<u32>, Vec<u32>) {
    let parts: Vec<&str> = input.split(':').collect();
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
    (winners, hands)
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
        let expected: (Vec<u32>, Vec<u32>) =
            (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]);
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
        assert_eq!(result, Some(30));
    }
}
