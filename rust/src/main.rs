mod lib;
mod day1;
mod day2;

use std::error::Error;
use std::path::Path;
use lib::Day;
use crate::lib::{Answer, Solution};

type SolverFn = fn() -> Result<Answer, Box<dyn Error>>;

fn solver_for(day: Day) -> SolverFn {
    match day {
        Day::ONE => day1::solve,
        Day::TWO => day2::solve,
        _ => unreachable!()
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let day = std::env::args().nth(1).expect("no day given");
    let day = day.parse::<u8>()?;
    let day = Day::from_int(day)?;

    let solver = solver_for(day);
    let answer = solver()?;
    let sol = Solution { day, answer };
    println!("{}", sol);

    Ok(())
}
