use nom::bytes::complete::{tag};
use nom::character::complete::{alphanumeric1};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use regex::{Regex};

pub fn run(data: String) {
    println!("Part 1 : {:?}", part1(&data));
    println!("Part 2 : {:?}", part2(&data));
}

fn get_first_digit_part1(input: &str) -> u32 {
    let regex = Regex::new(r"[1-9]").unwrap();
    let digits = regex.find_iter(input).map(|m| m.as_str().parse::<u8>().unwrap()).collect::<Vec<u8>>();
    format!("{}{}", digits.first().unwrap(), digits.last().unwrap()).parse::<u32>().unwrap()
}

fn process_part_1(input: &str) -> IResult<&str, u32> {
    let (input, value) = many1(alphanumeric1)(input)?;
    let processed = value.join("");
    Ok((input, get_first_digit_part1(processed.as_str())))
}

fn part1(data: &str) -> u32 {
    let (_, nums) = separated_list1(tag("\n"), process_part_1)(data).unwrap();
    nums.iter().sum()
}


fn get_first_digit_part2(input: &str) -> u32 {
    let regex = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    let mut digits : Vec<u8> = vec![];
    (0..input.len()).for_each(|n| {
        match regex.find_at(input, n){
            None => {}
            Some(x) => digits.push(match x.as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                x => x.parse::<u8>().unwrap()
            })
        }}
    );
    format!("{}{}", digits.first().unwrap(), digits.last().unwrap()).parse::<u32>().unwrap()
}


fn process_part_2(input: &str) -> IResult<&str, u32> {
    let (input, value) = many1(alphanumeric1)(input)?;
    let processed = value.join("");

    Ok((input, get_first_digit_part2(processed.as_str())))
}

fn part2(data: &str) -> u32 {
    let (_, nums) = separated_list1(tag("\n"), process_part_2)(data).unwrap();
    nums.iter().sum()
}