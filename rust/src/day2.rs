use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::lib::Answer;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

enum Outcome {
    Win,
    Lose,
    Draw
}

impl Outcome {
    fn value(&self) -> i8 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }

    fn choose(&self, opp: Choice) -> Choice {
        use Outcome::*;
        use Choice::*;
        match (self, opp) {
            (Win, Scissors) => Rock,
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Lose, Rock) => Scissors,
            (Lose, Paper) => Rock,
            (Lose, Scissors) => Paper,
            (Draw, _) => opp
        }
    }
}

#[derive(Debug)]
struct ParseOutcomeError;

impl Display for ParseOutcomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown outcome")
    }
}

impl Error for ParseOutcomeError {}

impl FromStr for Outcome {
    type Err = ParseOutcomeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(ParseOutcomeError)
        }
    }
}

#[derive(Debug)]
struct ParseChoiceError;

impl Display for ParseChoiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown choice")
    }
}

impl Error for ParseChoiceError {}

impl Choice {
    fn from_opp_str(s: &str) -> Result<Self, ParseChoiceError> {
        match s {
            "A" => Ok(Choice::Rock),
            "B" => Ok(Choice::Paper),
            "C" => Ok(Choice::Scissors),
            _ => Err(ParseChoiceError)
        }
    }

    fn from_my_str(s: &str) -> Result<Self, ParseChoiceError> {
        match s {
            "X" => Ok(Choice::Rock),
            "Y" => Ok(Choice::Paper),
            "Z" => Ok(Choice::Scissors),
            _ => Err(ParseChoiceError)
        }
    }

    fn value(&self) -> i8 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn beats(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper
        }
    }

    fn outcome(&self, opp: Choice) -> Outcome {
        if self.beats() == opp {
            Outcome::Win
        } else if opp.beats() == *self {
            Outcome::Lose
        } else {
            Outcome::Draw
        }
    }

    fn score(&self, opp: Choice) -> i8 {
        let outcome = self.outcome(opp);
        self.value() + outcome.value()
    }
}

struct Part1Round {
    my_choice: Choice,
    opp_choice: Choice
}

impl Part1Round {
    fn score(&self) -> i8 {
        self.my_choice.score(self.opp_choice)
    }
}

#[derive(Debug)]
enum ParseRoundError {
    UnknownOppChoice,
    UnknownMyChoice,
    NotEnoughElements,
}

impl Display for ParseRoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseRoundError::UnknownOppChoice => write!(f, "unknown opponent choice"),
            ParseRoundError::UnknownMyChoice => write!(f, "unknown my choice"),
            ParseRoundError::NotEnoughElements => write!(f, "not enough space-separated values on line")
        }
    }
}

impl Error for ParseRoundError {}

impl FromStr for Part1Round {
    type Err = ParseRoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let opp = split.next().ok_or(ParseRoundError::NotEnoughElements)?;
        let my = split.next().ok_or(ParseRoundError::NotEnoughElements)?;
        let opp_choice = Choice::from_opp_str(opp).map_err(|_| ParseRoundError::UnknownOppChoice)?;
        let my_choice = Choice::from_my_str(my).map_err(|_| ParseRoundError::UnknownMyChoice)?;
        Ok(Self {my_choice, opp_choice})
    }
}

const INP: &str = include_str!("../../day2_input.txt");

fn from_string(s: &str) -> Result<Vec<Part1Round>, ParseRoundError> {
    s.trim().lines().map(&str::parse::<Part1Round>).collect()
}

fn part1(rounds: &[Part1Round]) -> i64 {
    rounds.iter()
        .map(|round| round.score() as i64).sum()
}

struct Part2Round {
    opp_choice: Choice,
    desired: Outcome
}

impl Part2Round {
    fn score(&self) -> i8 {
        let my_choice = self.desired.choose(self.opp_choice);
        my_choice.score(self.opp_choice)
    }
}

/// Convert a round serialized into a Part1Round into a Part2Round.
fn convert_round(round: &Part1Round) -> Part2Round {
    let desired = match round.my_choice {
        Choice::Rock => Outcome::Lose,
        Choice::Paper => Outcome::Draw,
        Choice::Scissors => Outcome::Win,
    };
    Part2Round { opp_choice: round.opp_choice, desired }
}

fn part2(rounds: &[Part2Round]) -> i64 {
    rounds.iter()
        .map(|round| round.score() as i64)
        .sum()
}

pub fn solve() -> Result<Answer, Box<dyn Error>> {
    let p1_rounds = from_string(INP)?;
    let p2_rounds: Vec<_> = p1_rounds.iter().map(convert_round).collect();
    let (p1, p2) = (part1(&p1_rounds), part2(&p2_rounds));
    Ok(Answer {part_1: p1, part_2: p2})
}