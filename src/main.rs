mod day_eight;
mod day_five;
mod day_four;
mod day_nine;
mod day_one;
mod day_seven;
mod day_six;
mod day_three;
mod day_two;
mod file;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(day) = args.get(1) {
        let (_, day_input) = day.split_once("=").unwrap();
        match day_input {
            "1" => day_one::solve(),
            "2" => day_two::solve(),
            "3" => day_three::solve(),
            "4" => day_four::solve(),
            "5" => day_five::solve(),
            "6" => day_six::solve(),
            "7" => day_seven::solve(),
            "8" => day_eight::solve(),
            "9" => day_nine::solve(),
            _ => eprintln!("Invalid day"),
        }
    } else {
        eprintln!("USAGE: {} <file> <width>", env::args().next().unwrap());
    }
}
