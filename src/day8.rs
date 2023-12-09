use std::collections::HashMap;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::error::Error;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{terminated};
use rayon::prelude::*;

pub fn run(data: String) {
    println!("Part 1 : {:?}", part1(&data));
    println!("Part 2 : {:?}", part2(&data));
}

fn parse_field (input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (input, key) = terminated(alphanumeric1::<&str, Error<&str>>, tag(" = ("))(input).unwrap();
    let (input, left) = terminated(alphanumeric1::<&str, Error<&str>>, tag(", "))(input).unwrap();
    let (input, right) = terminated(alphanumeric1::<&str, Error<&str>>, tag(")"))(input).unwrap();
    Ok((input, (key, left, right)))
}

fn parse_map(input: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
    let (input, fields) = separated_list1(tag("\n"), parse_field)(input).unwrap();
    let built_map = fields.into_iter().fold(HashMap::<&str, (&str, &str)>::new(), |mut map, (key, left, right)| {
        map.insert(key, (left, right));
        map
    });
    Ok((input, built_map))
}

fn process_part_1(directions: &str, map: HashMap<&str, (&str, &str)>) -> usize {
    let mut current = "AAA";
    for (i, c) in directions.chars().cycle().enumerate() {
        if current == "ZZZ" {
            return i
        }
        let field = map.get(current).unwrap();
        current = match c {
            'L' => field.0,
            'R' => field.1,
            _ => panic!("Should never happen")
        }
    }
    0
}

fn part1(data: &String) -> usize {
    let (input, directions) = terminated(alphanumeric1::<&str, Error<&str>>, tag("\n\n"))(data.as_str()).unwrap();
    let (_, map) = parse_map(input).unwrap();
    process_part_1(directions, map)
}

fn process_part_2(directions: &str, map: HashMap<&str, (&str, &str)>) -> u64 {
    let starts = map.keys().filter(|x| x.ends_with('A')).copied().collect::<Vec<&str>>();
    let x = starts.into_par_iter().map(|start| {
        let mut current = start;
        for (i, c) in directions.chars().cycle().enumerate() {
            if current.ends_with('Z') {
                return i as u64
            }
            let field = map.get(current).unwrap();
            current = match c {
                'L' => field.0,
                'R' => field.1,
                _ => panic!("Should never happen")
            }
        }
        0u64
    }).collect::<Vec<u64>>();

    x.iter().skip(1).fold(*x.first().unwrap(), |res, y| {
        num::integer::lcm(res, *y)
    })
}

fn part2(data: &String) -> u64 {
    let (input, directions) = terminated(alphanumeric1::<&str, Error<&str>>, tag("\n\n"))(data.as_str()).unwrap();
    let (_, map) = parse_map(input).unwrap();

    process_part_2(directions, map)
}