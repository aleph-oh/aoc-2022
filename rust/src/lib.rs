#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Day(pub u8);

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day {}", self.0)
    }
}

impl Day {
    pub const ONE: Day = Day(1);
    pub const TWO: Day = Day(2);

    const MIN_DAY: u8 = 1;
    const MAX_DAY: u8 = 2;

    pub const fn from_int(n: u8) -> Result<Self, InvalidDay> {
        if Day::MIN_DAY <= n && n <= Day::MAX_DAY {
            Ok(Day(n))
        } else {
            Err(InvalidDay(n))
        }
    }

    pub fn input_path(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(format!("../../day{}_input.txt", self.0))
    }
}

#[derive(Debug)]
pub struct InvalidDay(u8);

impl std::fmt::Display for InvalidDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid day: {}", self.0)
    }
}

impl std::error::Error for InvalidDay {}


pub struct Answer {
    pub part_1: i64,
    pub part_2: i64
}


pub struct Solution {
    pub day: Day,
    pub answer: Answer
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n\tpart 1: {}\n\tpart 2: {}", self.day, self.answer.part_1, self.answer.part_2)
    }
}
