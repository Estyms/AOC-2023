use itertools::Itertools;
use num::abs;

pub fn run(data: String) {
    println!("Part 1 : {:?}", part1(&data));
    println!("Part 2 : {:?}", part2(&data));
}

#[derive(Debug)]
struct Galaxy {
    x: i64,
    y: i64,
}

fn process_galaxies(input: &str, expand_ratio: i64) -> Vec<Galaxy> {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut galaxies = lines.iter().enumerate().flat_map(
        |(y, line)| {
            line.chars().enumerate().filter(|(_, char)| *char == '#')
                .map(move |(x, _)|
                    Galaxy {
                        x: x as i64,
                        y: y as i64,
                    }
                ).collect::<Vec<Galaxy>>()
        }).collect::<Vec<Galaxy>>();

    let max_x = galaxies.iter().map(|g| g.x).max().unwrap();
    let max_y = galaxies.iter().map(|g| g.y).max().unwrap();

    for i in (0..max_x).rev() {
        if galaxies.iter().all(|g| g.x != i) {
            galaxies.iter_mut().filter(|g| g.x > i).for_each(|g| {
                g.x += expand_ratio - 1;
            })
        }
    }

    for i in (0..max_y).rev() {
        if galaxies.iter().all(|g| g.y != i) {
            galaxies.iter_mut().filter(|g| g.y > i).for_each(|g| {
                g.y += expand_ratio - 1;
            })
        }
    }

    galaxies
}

fn manhattan_distance(a: &Galaxy, b: &Galaxy) -> i64 {
    abs(a.x - b.x) + abs(a.y - b.y)
}

fn calculate_paths(galaxies: Vec<Galaxy>) -> Vec<i64> {
    let combinations = galaxies.iter().combinations(2).collect::<Vec<Vec<&Galaxy>>>();
    combinations.iter().map(|vec| {
        manhattan_distance(vec.first().unwrap(), vec.last().unwrap())
    }).collect()
}

fn part1(data: &String) -> i64 {
    let galaxies = process_galaxies(data.as_str(), 2);
    calculate_paths(galaxies).iter().sum()
}

fn part2(data: &String) -> i64 {
    let galaxies = process_galaxies(data.as_str(), 1_000_000);
    calculate_paths(galaxies).iter().sum()
}