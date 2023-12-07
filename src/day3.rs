use std::collections::HashMap;
use regex::{Regex};

pub fn run(data: String) {
    println!("Part 1 : {:?}", part1(&data));
    println!("Part 2 : {:?}", part2(&data));
}


#[derive(Debug, Clone)]
struct Number {
    x: i64,
    y: i64,
    length: i64,
    value: u64
}

#[derive(Debug, Clone)]
struct Symbol {
    x : i64,
    y: i64
}

fn process_part_1(input: &str, row: usize) -> (Vec<Number>, Vec<Symbol>) {
    let number_regex = Regex::new(r"[0-9]+").unwrap();
    let x : Vec<Number> = number_regex.captures_iter(input).map(|c| {
        let capture = c.get(0).unwrap();
        Number { x: capture.start() as i64,y: row as i64, length : capture.len() as i64, value: capture.as_str().parse().unwrap()}
    }).collect();

    let symbol_regex = Regex::new(r"(?m)[^\d.\n]").unwrap();
    let y : Vec<Symbol> = symbol_regex.captures_iter(input).map(|c| {
        let capture = c.get(0).unwrap();
        Symbol {x: capture.start() as i64, y: row as i64}
    }).collect();

    (x, y)
}

fn is_adjacent_to_symbol(number: &Number, symbols: &[Symbol]) -> bool {
    symbols.iter().rfold(false, |res, s| {
        res | (
                    s.x <= (number.x + number.length)
                &&  s.x >= number.x - 1
                &&  s.y <= number.y + 1
                &&  s.y >= number.y -1
        )
    })
}

fn get_numbers_and_symbols(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut i : usize = 0;
    let x: Vec<(Vec<Number>, Vec<Symbol>)> = input.split('\n').map(|x| {i+=1; process_part_1(x, i-1)}).collect();
    x.into_iter().rfold(( vec![], vec![]), |(ln,ls), (n, s)| {
        ([ln, n].concat(), [ls, s].concat())
    })
}

fn part1(input: &String) -> u64 {
    let (numbers, symbols) = get_numbers_and_symbols(input);
    numbers.into_iter().filter(|num| is_adjacent_to_symbol(num, &symbols)).map(|num| num.value).sum()
}


fn process_symbols(number: &Number, symbols: &[Symbol], hashmap: & mut HashMap<(i64, i64), Vec<Number>>) {
    let symbol = symbols.iter().find(|s| {

            s.x <= (number.x + number.length)
                &&  s.x >= number.x - 1
                &&  s.y <= number.y + 1
                &&  s.y >= number.y -1

    });

    match symbol {
        None => {}
        Some(x) => {
            match hashmap.get_mut(&(x.x, x.y)) {
                None => {hashmap.insert((x.x, x.y), vec![number.clone()]); }
                Some(e) => {
                    e.push(number.clone());
                }
            }
        }
    }
}

fn part2(input: &String) -> u64 {
    let (numbers, symbols) = get_numbers_and_symbols(input);
    let mut map: HashMap<(i64, i64), Vec< Number >> = HashMap::new();
    numbers.into_iter().for_each(|num| {process_symbols(&num, &symbols, &mut map)});
    map.iter().filter(|(_, val)| val.len() == 2).map(|p| p.1.iter().map(|p| p.value).product::<u64>()).sum()
}