use std::cmp::Ordering;
use std::collections::HashMap;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alphanumeric1, space1};
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use rayon::prelude::*;
use crate::day7::HandType::*;


pub fn run(data: String) {
    println!("Part 1 : {:?}", part1(&data));
    println!("Part 2 : {:?}", part2(&data));
}


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard = 1,
    Pair = 2,
    DoublePair = 3,
    Three = 4,
    FullHouse = 5,
    Four = 6,
    Five = 7,
}

#[derive(Debug)]
struct Hand {
    score: u64,
    hand_type: HandType,
    hand: Vec<u8>,
}

impl Hand {
    fn get_hand_type(hand_vec: &[u8]) -> HandType {
        let card_map = hand_vec.iter().fold(HashMap::new(), |mut hash_map: HashMap<u8, u8>, card| {
            if let Some(x) = hash_map.get_mut(card) {
                *x += 1;
            } else {
                hash_map.insert(*card, 1);
            }
            hash_map
        });

        let kv: Vec<(u8, u8)> = card_map.keys().fold(vec![], |mut vec: Vec<(u8, u8)>, k| {
            let (k, v) = card_map.get_key_value(k).unwrap();
            vec.push((*k, *v));
            vec.sort_by(|(a, _), (b, _)| b.cmp(a));
            vec
        });

        if kv.len() == 1 {
            Five
        } else if get_first_n_valued_key(4, &kv).is_some() {
            Four
        } else if let (Some(_), Some(_)) = (
            get_first_n_valued_key(3, &kv), get_first_n_valued_key(2, &kv))
        {
            FullHouse
        } else if get_first_n_valued_key(3, &kv).is_some() {
            Three
        } else if kv.iter().filter(|(_, v)| *v == 2).count() == 2 {
            DoublePair
        } else if get_first_n_valued_key(2, &kv).is_some() {
            Pair
        } else {
            HighCard
        }
    }

    fn get_card_value_p2(card: char) -> u8 {
        "J23456789TQKA".find(card).unwrap() as u8
    }

    fn get_possible_cards(hand: &str) -> Vec<u8> {
        hand.chars().map(Hand::get_card_value_p2).collect()
    }

    fn str_to_hand_vec_p1(hand: &str) -> Vec<u8> {
        hand.chars().map(|x| "23456789TJQKA".find(x).unwrap() as u8).collect()
    }

    fn str_to_hand_vec_p2(hand: &str) -> Vec<u8> {
        hand.chars().map(Hand::get_card_value_p2).collect()
    }

    fn new_p1(hand_str: &str, score: u64) -> Hand {
        let hand = Hand::str_to_hand_vec_p1(hand_str);

        Hand {
            hand_type: Hand::get_hand_type(&hand),
            hand,
            score,
        }
    }

    fn new_p2(hand_str: &str, score: u64) -> Hand {
        let hand = Hand::str_to_hand_vec_p2(hand_str);
        let mut card_types = Hand::get_possible_cards(hand_str);
        card_types.dedup();

        let hands = hand.iter().fold(vec![], |mut hands: Vec<Vec<u8>>, card| {
            match card {
                0 => {
                    card_types
                        .iter()
                        .map(|c|
                            match hands.len() {
                                0 => {
                                    vec![vec![*c]]
                                }
                                _ => hands
                                    .iter()
                                    .map(|h| {
                                        let mut x = h.clone();
                                        x.push(*c);
                                        x
                                    })
                                    .collect::<Vec<Vec<u8>>>()
                            }
                        ).fold(vec![], |mut res, mut h| {
                        res.append(&mut h);
                        res
                    })
                }
                x => {
                    match hands.len() {
                        0 => hands.push(vec![*x]),
                        _ => hands.iter_mut().for_each(|h| { h.push(*x); })
                    }
                    hands
                }
            }
        });

        Hand {
            hand_type: hands.par_iter().map(|x| Hand::get_hand_type(x)).max().unwrap(),
            hand,
            score,
        }
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.hand.eq(&other.hand)
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.partial_cmp(&other.hand_type).unwrap() {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                self.hand.cmp(&other.hand)
            }
            Ordering::Greater => Ordering::Greater
        }
    }
}


fn get_first_n_valued_key(n: u8, vec: &[(u8, u8)]) -> Option<u8> {
    vec.iter().filter(|(_, v)| *v == n).map(|(k, _)| *k).collect::<Vec<u8>>().first().copied()
}

fn process_hands(input: &str) -> Vec<(&str, u64)> {
    let (_, hands): (&str, Vec<(&str, u64)>) = separated_list1(tag("\n"), separated_pair(alphanumeric1::<&str, Error<&str>>, space1, complete::u64))(input).unwrap();
    hands
}

fn part1(data: &str) -> u64 {
    let hands = process_hands(data);
    let mut hands: Vec<Hand> = hands.iter().map(|h| Hand::new_p1(h.0, h.1)).collect();
    hands.sort();
    hands.iter().enumerate().map(|(i, h)| ((i + 1) as u64) * h.score).sum()
}

fn part2(data: &str) -> u64 {
    let hands = process_hands(data);
    let mut hands: Vec<Hand> = hands.iter().map(|h| Hand::new_p2(h.0, h.1)).collect();
    hands.sort();
    hands.iter().enumerate().map(|(i, h)| ((i + 1) as u64) * h.score).sum()
}