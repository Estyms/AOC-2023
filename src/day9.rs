use nom::bytes::complete::tag;
use nom::error::{Error};
use nom::multi::separated_list1;

pub fn run(data: String) {
    println!("Part 1 : {:?}", part1(&data));
    println!("Part 2 : {:?}", part2(&data));
}

fn get_reduction(list: &[i32]) -> Vec<i32> {
    list.windows(2).map(|w| {
        w.last().unwrap() - w.first().unwrap()
    }).collect()
}

fn get_reduction_list(mut prec: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if prec.last().unwrap().iter().sum::<i32>() == 0 {
        return prec
    }
    prec.push(get_reduction(prec.last().unwrap()));
    get_reduction_list(prec)
}

fn process_part_1(list: &[Vec<i32>]) -> i32 {
    list.iter().map(|init| {
        let reduction_map: Vec<Vec<i32>> = vec![init.clone()];
        let reduced_map = get_reduction_list(reduction_map);
        reduced_map.iter().rfold(0, |last, lst| lst.last().unwrap() + last)
    }).collect::<Vec<i32>>().iter().sum()
}

fn process_list(input: &str) ->  Vec<Vec<i32>> {
    separated_list1(tag::<&str, &str, Error<&str>>("\n"), separated_list1(tag(" "), nom::character::complete::i32))(input).unwrap().1
}

fn part1(data: &str) -> i32 {
    process_part_1(&process_list(data))
}


fn process_part_2(list: &[Vec<i32>]) -> i32 {
    list.iter().map(|init| {
        let reduction_map: Vec<Vec<i32>> = vec![init.clone()];
        let reduced_map = get_reduction_list(reduction_map);
        reduced_map.iter().rfold(0, |last, lst| lst.first().unwrap() - last)
    }).collect::<Vec<i32>>().iter().sum()
}

fn part2(data: &str) -> i32 {
    process_part_2(&process_list(data))
}