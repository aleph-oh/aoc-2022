mod lib;
mod day1;

use std::error::Error;
use std::path::Path;
use lib::Day;
use crate::lib::{Answer, Solution};

type SolverFn = fn(&Path) -> Result<Answer, Box<dyn Error>>;

const DAY_ONE: Day = Day(1);

fn solver_for(day: Day) -> SolverFn {
    match day {
        DAY_ONE => day1::solve,
        _ => unreachable!()
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let day = std::env::args().nth(1).expect("no day given");
    let day = day.parse::<u8>()?;
    let day = Day::from_int(day)?;

    let solver = solver_for(day);
    let answer = solver(&day.input_path())?;
    let sol = Solution { day, answer };
    println!("{}", sol);

    Ok(())
}
