mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

use std::fs;
use inquire::Text;

fn main() {
    let day = Text::new("What day do you want to run?\n>").prompt();

    match day {
        Ok(day) => {
            match day.parse::<i32>().expect("The day entered is invalid") {
                x if x < 32 && x > 0 => run_day(x),
                _ => println!("The day entered is invalid")
            }
        }
        Err(_) => {
            println!("The day entered is invalid")
        }
    }

}

type DayFn = Box<dyn Fn(String)>;
fn run_day(day: i32) {
    let map : [(i32, DayFn); 10] = [
        (1, Box::from(day1::run)),
        (2, Box::from(day2::run)),
        (3, Box::from(day3::run)),
        (4, Box::from(day4::run)),
        (5, Box::from(day5::run)),
        (6, Box::from(day6::run)),
        (7, Box::from(day7::run)),
        (8, Box::from(day8::run)),
        (9, Box::from(day9::run)),
        (10, Box::from(day10::run))
    ];

    let data = fs::read_to_string(format!("inputs/day{}.txt", day).as_str()).expect("Can't find that day input file");

    for (list_day, func) in map {
        if list_day == day {
            func(data);
            return;
        }
    }
    println!("Day isn't implemented");
}