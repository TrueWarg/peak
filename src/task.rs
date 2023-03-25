use std::{fmt::Display};

pub trait Question {
    type Body: Display;
    type Solution;
    fn body(&self) -> Self::Body;
    fn solution(&self) -> Self::Solution;
}

pub fn default_check<Q: Question>(solution: Q::Solution, answer: Q::Solution) -> bool
where
    Q::Solution: PartialEq,
{
    return solution == answer;
}
