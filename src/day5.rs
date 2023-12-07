use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, multispace1, newline, space1};
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated, tuple};
use nom::IResult;
use rayon::prelude::*;

pub fn run(data: String) {
    println!("Part 1 : {:?}", part1(&data));
    println!("Part 2 : {:?}", part2(&data));
}

#[derive(Debug)]
struct Data {
    start_source: u64,
    start_destination: u64,
    range: u64,
}

#[derive(Debug)]
struct Map {
    data: Vec<Data>,
}

impl Map {
    fn find_dest_number(&self, src: u64) -> u64 {
        if let Some(d) = self
            .data
            .iter()
            .find(|d| (d.start_source..(d.start_source + d.range)).contains(&src))
        {
            d.start_destination + (src - d.start_source)
        } else {
            src
        }
    }
}

fn process_data(input: &str) -> IResult<&str, Data> {
    let (input, numbers): (&str, Vec<u64>) =
        separated_list1(space1::<&str, Error<&str>>, nom::character::complete::u64)(input).unwrap();
    Ok((
        input,
        Data {
            start_destination: *numbers.first().unwrap(),
            start_source: *numbers.get(1).unwrap(),
            range: *numbers.get(2).unwrap(),
        },
    ))
}

fn process_map(input: &str) -> IResult<&str, Map> {
    let (input, _): (&str, &str) = alpha1::<&str, Error<&str>>(input).unwrap();
    let (input, _) = tag::<&str, &str, Error<&str>>("-to-")(input).unwrap();
    let (input, _) = terminated(
        alpha1::<&str, Error<&str>>,
        tuple((space1, tag("map:"), newline)),
    )(input)
    .unwrap();
    let res = take_until::<&str, &str, Error<&str>>("\n\n")(input);

    if let Ok((input, numbers)) = res {
        let (_, data) = separated_list1(newline, process_data)(numbers).unwrap();
        Ok((input, Map { data }))
    } else {
        let (_, data) = separated_list1(newline, process_data)(input).unwrap();
        Ok((input, Map { data }))
    }
}

fn process_maps(input: &str) -> IResult<&str, Vec<Map>> {
    let (input, maps): (&str, Vec<Map>) = separated_list1(multispace1, process_map)(input).unwrap();
    Ok((input, maps))
}

fn seed_locations(seed: u64, maps: &Vec<Map>) -> u64 {
    maps.iter().fold(seed, |num, m| m.find_dest_number(num))
}

fn part1(data: &String) -> u64 {
    let (input, _) = tag::<&str, &str, Error<&str>>("seeds: ")(data.as_str()).unwrap();
    let (input, seeds): (&str, Vec<u64>) =
        separated_list1(space1::<&str, Error<&str>>, nom::character::complete::u64)(input).unwrap();
    let (input, _) = multispace1::<&str, Error<&str>>(input).unwrap();
    let (_, maps) = process_maps(input).unwrap();
    seeds
        .into_iter()
        .map(|x| seed_locations(x, &maps))
        .collect::<Vec<u64>>()
        .into_iter()
        .min()
        .unwrap()
}

fn part2(data: &String) -> u64 {
    let (input, _) = tag::<&str, &str, Error<&str>>("seeds: ")(data.as_str()).unwrap();
    let (input, seed_ranges): (&str, Vec<(u64, u64)>) = separated_list1(
        space1::<&str, Error<&str>>,
        separated_pair(
            nom::character::complete::u64,
            space1,
            nom::character::complete::u64,
        ),
    )(input)
    .unwrap();
    let (input, _) = multispace1::<&str, Error<&str>>(input).unwrap();
    let (_, maps) = process_maps(input).unwrap();
    seed_ranges
        .into_iter()
        .map(|(start, range)| {
            (start..start + range)
                .into_par_iter()
                .map(|y| seed_locations(y, &maps))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}
