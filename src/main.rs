mod day1;
mod day2;
mod day3;
mod day4;

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
    let map : [(i32, DayFn); 4] = [
        (1, Box::from(day1::run)),
        (2, Box::from(day2::run)),
        (3, Box::from(day3::run)),
        (4, Box::from(day4::run))
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