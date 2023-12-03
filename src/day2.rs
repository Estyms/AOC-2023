use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::error::{Error};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use crate::day2::PullColor::*;

pub fn run(data: String) {
    println!("Part 1 : {:?}", part1(&data));
    println!("Part 2 : {:?}", part2(&data));
}

#[derive(Debug)]
enum PullColor {
    Red(u64),
    Green(u64),
    Blue(u64)
}

#[derive(Debug)]
struct Pull {
    red: Option<u64>,
    green: Option<u64>,
    blue: Option<u64>
}

fn get_pulls(input: &str) -> (u64,Vec<Pull>) {
    let (input, id) : (&str, u64) = delimited(tag("Game "), nom::character::complete::u64::<&str, Error<&str>>, tag(": "))(input).unwrap();
    (id, input.split("; ").map(|p| {
        let (_, pull_colors) = separated_list1(tag(", "),  process_pull)(p).unwrap();
        let mut pull = Pull {
            red: None,
            green: None,
            blue: None,
        };

        for pull_color in pull_colors {
            match pull_color {
                Red(x) => {pull.red = Some(x)}
                Green(x) => {pull.green = Some(x)}
                Blue(x) => {pull.blue = Some(x)}
            }
        }
        pull
    }).collect())
}

fn process_pull(input: &str) -> IResult<&str, PullColor> {
    let (input, number) : (&str, u64) = nom::character::complete::u64::<&str, Error<&str>>(input).unwrap();
    let (input, _) = multispace1::<&str, Error<&str>>(input).unwrap();
    let (input, color) : (&str, &str) = alt((tag::<&str, &str, Error<&str>>("green"), tag("red"), tag("blue")))(input).unwrap();
    let pull_color = match color {
        "red" => Red(number),
        "green" => Green(number),
        "blue" => Blue(number),
        _ => panic!("Color unknown")
    };
    Ok((input, pull_color))
}

fn process_part_1(input: &str) -> (u64, bool) {
    let (id, pulls) = get_pulls(input);

    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    let result = pulls.iter().rfold(true, |res, p| {
        if let Some(red) = p.red {
            if red > 12 {return false;}
        };
        if let Some(green) = p.green {
            if green > 13 {return false;}
        };
        if let Some(blue) = p.blue {
            if blue > 14 {return false;}
        };
        res & true
    });
    (id, result)
}

fn part1(data: &String) -> u64 {
    let lines: Vec<(u64, bool)> = data.split("\n").map(|f| process_part_1(f)).collect();
    lines.iter().filter(|(_, x)| *x).map(|(id, _)| id).sum()
}


fn process_part_2(input: &str) -> u64 {
    let (_, pulls) = get_pulls(input);
    let mut min_pull = Pull {
        red: None,
        green: None,
        blue: None,
    };

    pulls.iter().for_each(|p| {
        if let Some(red) = p.red {
            match min_pull.red {
                Some(x) if x > red => {},
                _ => {min_pull.red = Some(red)}
            }
        };
        if let Some(green) = p.green {
            match min_pull.green {
                Some(x) if x > green => {}
                _ => {min_pull.green = Some(green)}
            }
        };
        if let Some(blue) = p.blue {
            match min_pull.blue {
                Some(x) if x > blue => {}
                _ => {min_pull.blue = Some(blue)}
            }
        };
    });
    min_pull.red.unwrap() * min_pull.green.unwrap() * min_pull.blue.unwrap()
}

fn part2(data: &String) -> u64 {
    data.split("\n").map(|f| process_part_2(f)).sum()
}