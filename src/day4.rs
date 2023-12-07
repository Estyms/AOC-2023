use std::collections::HashMap;
use nom::branch::permutation;
use nom::bytes::complete::tag;
use nom::character::complete::{space1};
use nom::error::{Error};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::preceded;

pub fn run(data: String) {
    println!("Part 1 : {:?}", part1(&data));
    println!("Part 2 : {:?}", part2(&data));
}

#[derive(Debug)]
struct Card {
    id: u64,
    elf_numbers: Vec<u64>,
    win_numbers: Vec<u64>
}

impl Card {
    fn count_winning_numbers(&self) -> u64 {
        self.win_numbers.iter().filter(|n| self.elf_numbers.contains(n)).count() as u64
    }
}

fn num_list(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, numbers) : (&str, Vec<u64>) = separated_list1(space1, nom::character::complete::u64::<&str, Error<&str>>)(input).unwrap();
    Ok((input, numbers))
}
fn card_processing(input : &str) -> IResult<&str, Card> {
    let (input, id) : (&str, u64) = preceded(permutation((tag("Card"), space1)), nom::character::complete::u64::<&str, Error<&str>>)(input).unwrap();
    let (input, win_numbers) = preceded(permutation((tag(":"), space1)), num_list)(input).unwrap();
    let (input, _) = permutation((space1::<&str, Error<&str>>, tag("|"), space1))(input).unwrap();
    let (input, elf_numbers) = num_list(input).unwrap();
    Ok((input, Card {id, win_numbers, elf_numbers}))
}

fn part1(input: &String) -> u64 {
    let (_, cards) = separated_list1(tag("\n"), card_processing)(input).unwrap();
    cards.iter().map(|c| match c.count_winning_numbers() {
        0 => 0,
        x => 2_u64.pow((x - 1) as u32)
    }).sum()
}


fn game_loop(card_map: &mut HashMap<u64, u64>, cards: Vec<Card>) -> u64 {
    let mut final_card_map : HashMap<u64, u64> = card_map.clone();
    while card_map.iter().any(|(_, x)| *x != 0) {
        let old_card_map = card_map.clone();
        old_card_map.iter().for_each(|(id, card_count)| {
            match card_count {
                0 => {}
                number => {
                    let card : &Card = cards.iter().find(|c| c.id == *id).unwrap();
                    let winning = card.count_winning_numbers();
                    for i in *id+1..*id+winning+1 {
                        let reference = card_map.get_mut(&i).unwrap();
                        *reference += number;

                        let reference = final_card_map.get_mut(&i).unwrap();
                        *reference += number;
                    }
                    let reference = card_map.get_mut(id).unwrap();
                    *reference -= number;
                }
            }
        } );
    }
    final_card_map.values().sum()
}


fn part2(input: &String) -> u64 {
    let (_, cards) = separated_list1(tag("\n"), card_processing)(input).unwrap();
    let mut card_map : HashMap<u64, u64> = HashMap::new();
    cards.iter().for_each(|x| {card_map.insert(x.id, 1);});
    game_loop(&mut card_map, cards)
}