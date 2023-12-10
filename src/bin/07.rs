use advent_of_code_2023::shared::{PartSolution, Parts};

advent_of_code_2023::solution!();

#[derive(Debug, PartialEq, Eq)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err(format!(
                "Invalid character: {} is not part of a deck",
                value
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

fn parse_hands(input: &str) -> Vec<Hand> {
    let mut hands = vec![];
    for line in input.lines() {
        let (game, bid) = line.split_once(' ').expect("Invalid game");

        let parsed_game = game
            .chars()
            .map(|c| c.try_into().expect("Invalid character"))
            .collect::<Vec<Card>>();

        let parsed_bid = bid.parse().expect("Invalid bid");
        hands.push(Hand {
            cards: parsed_game,
            bid: parsed_bid,
        });
    }

    hands
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let _hands = parse_hands(input);

        PartSolution::None
    }

    fn part_2(&self, _input: &str) -> PartSolution {
        PartSolution::None
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2023::shared::{solution::read_file, PartSolution, Parts};

        use crate::{parse_hands, Card, Hand, Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_1(&read_file("examples", &DAY))
            );
        }

        #[test]
        fn parse() {
            let input = [
                "32T3K 765",
                "T55J5 684",
                "KK677 28",
                "KTJJT 220",
                "QQQJA 483",
            ]
            .join("\n");

            let hands = parse_hands(input.as_str());

            assert_eq!(
                vec![
                    Hand {
                        cards: vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                        bid: 765
                    },
                    Hand {
                        cards: vec![Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five],
                        bid: 684
                    },
                    Hand {
                        cards: vec![Card::King, Card::King, Card::Six, Card::Seven, Card::Seven],
                        bid: 28
                    },
                    Hand {
                        cards: vec![Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten],
                        bid: 220
                    },
                    Hand {
                        cards: vec![Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
                        bid: 483
                    }
                ],
                hands
            );
        }
    }

    mod part_2 {
        use advent_of_code_2023::shared::{solution::read_file, PartSolution, Parts};

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
