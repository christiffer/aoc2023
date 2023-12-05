use std::cmp::max;
use std::collections::HashMap;

advent_of_code::solution!(2);

const GAME_BAG: &str = "12 red, 13 green, 14 blue";

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let game = parse_game(line);
        let actual = is_game_possible(&game, GAME_BAG);
        sum += actual;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut power = 0;
    for line in lines {
        let game = parse_game(line);
        let (_, actual) = lowest_possible_bag(&game);
        power += actual;
    }
    Some(power)
}

fn is_game_possible(game: &Game, blocks: &str) -> u32 {
    let blocks = parse_blocks(blocks);

    let valid = game
        .blocks
        .iter()
        .map(|b| is_hand_valid(b, &blocks))
        .reduce(|l, r| l && r)
        .unwrap();

    return if valid { game.id } else { 0 };
}

fn lowest_possible_bag(game: &Game) -> (Blocks, u32) {
    let blocks: Vec<HashMap<String, u32>> = game.blocks.iter().map(|b| b.value.clone()).collect();
    let mut value: HashMap<String, u32> = HashMap::new();

    for game_hand in blocks {
        for (k, v) in game_hand.iter() {
            if value.contains_key(k) {
                let vv = value.get(k).unwrap();
                let newv = max(*v, *vv);
                value.insert(k.to_string(), newv);
            } else {
                value.insert(k.to_string(), *v);
            }
        }
    }

    let smallest_bag = Blocks {
        value: value.clone(),
    };

    let power = value.values().product();
    (smallest_bag, power)
}

fn is_hand_valid(hand: &Blocks, state: &Blocks) -> bool {
    let mut valid = true;
    for key in hand.value.keys() {
        valid &= hand.value[key] <= state.value[key];
    }
    return valid;
}

#[derive(Eq, Debug, PartialEq)]
struct Game {
    id: u32,
    blocks: Vec<Blocks>,
}

#[derive(Eq, Debug, PartialEq)]
struct Blocks {
    value: HashMap<String, u32>,
}

fn parse_game(game_string: &str) -> Game {
    let splits: Vec<&str> = game_string.split(":").collect();
    let game_string = splits[0].trim();
    let game_id = game_string.replace("Game ", "").parse::<u32>().unwrap();
    let hands: Vec<&str> = splits[1].split(";").collect();

    let mut blocks: Vec<Blocks> = Vec::new();

    for hand in hands {
        let block = parse_blocks(hand);
        blocks.push(block);
    }

    Game {
        id: game_id,
        blocks,
    }
}

fn parse_blocks(hand_state: &str) -> Blocks {
    let mut values: HashMap<String, u32> = HashMap::new();
    hand_state.split(",").for_each(|hand| {
        let splits: Vec<&str> = hand.trim().split(" ").collect();
        let val: u32 = splits[0].parse::<u32>().unwrap();
        let key: &str = splits[1];
        values.insert(key.to_owned(), val);
    });
    Blocks { value: values }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowest_possible_bag() {
        let game = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let expected = Blocks {
            value: HashMap::from([
                ("blue".to_string(), 6),
                ("red".to_string(), 4),
                ("green".to_string(), 2),
            ]),
        };
        let expected_power: u32 = 48;
        let actual = lowest_possible_bag(&game);
        assert_eq!(actual, (expected, expected_power));
    }

    #[test]
    fn test_parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = parse_game(input);
        let actual = Game {
            id: 1,
            blocks: Vec::from([
                Blocks {
                    value: HashMap::from([(String::from("blue"), 3), (String::from("red"), 4)]),
                },
                Blocks {
                    value: HashMap::from([
                        (String::from("red"), 1),
                        (String::from("green"), 2),
                        (String::from("blue"), 6),
                    ]),
                },
                Blocks {
                    value: HashMap::from([(String::from("green"), 2)]),
                },
            ]),
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_blocks() {
        let input: &str = "3 blue, 4 red";

        let expected = Blocks {
            value: HashMap::from([(String::from("blue"), 3), (String::from("red"), 4)]),
        };

        let actual = parse_blocks(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_hand_valid() {
        let blocks = Blocks {
            value: HashMap::from([
                (String::from("blue"), 1),
                (String::from("red"), 1),
                (String::from("green"), 1),
            ]),
        };
        let is_valid: bool = is_hand_valid(&blocks, &blocks);

        assert_eq!(is_valid, true);
    }

    #[test]
    fn test_is_hand_invalid() {
        let hand = Blocks {
            value: HashMap::from([
                (String::from("blue"), 2),
                (String::from("red"), 1),
                (String::from("green"), 1),
            ]),
        };
        let blocks = Blocks {
            value: HashMap::from([
                (String::from("blue"), 1),
                (String::from("red"), 1),
                (String::from("green"), 1),
            ]),
        };
        let is_valid: bool = is_hand_valid(&hand, &blocks);

        assert_eq!(is_valid, false);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
