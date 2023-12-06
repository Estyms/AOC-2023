use std::iter::zip;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{multispace1, space1};
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};

pub fn run(data: String) {
    println!("Part 1 : {:?}", part1(&data));
    println!("Part 2 : {:?}", part2(&data));
}

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64
}

fn num_of_records(race: &Race) -> u64 {
    let range = 1..race.time;
    range.map(|time_pressed| {
        (race.time - time_pressed) * time_pressed
    }).filter(|d| *d > race.record).count() as u64
}

fn parse_data(input: &str) -> (Vec<u64>, Vec<u64>) {
    let (input, times) : (&str, Vec<u64>) = preceded(tuple((tag("Time:"), space1::<&str, Error<&str>>)), separated_list1(space1, complete::u64))(input).unwrap();
    let (_, distances) : (&str, Vec<u64>) = preceded(tuple((multispace1, tag("Distance:"), space1::<&str, Error<&str>>)), separated_list1(space1, complete::u64))(input).unwrap();
    (times, distances)
}

fn part1(data: &String) -> u64 {
    let (times, distances) = parse_data(data.as_str());
    let races = zip(times, distances).map(|(time, record)| {Race {time, record}}).collect::<Vec<Race>>();
    races.iter().map(num_of_records).product()
}

fn part2(data: &String) -> u64 {
    let (times, distances) = parse_data(data.as_str());
    let race_data = zip(times, distances).fold((String::from(""), String::from("")), |(ts, ds), (t, d)| {
        (format!("{}{}", ts, t),
            format!("{}{}", ds, d))
    });
    let race = Race {
        time: race_data.0.parse().unwrap(),
        record: race_data.1.parse().unwrap()
    };
    num_of_records(&race)
}