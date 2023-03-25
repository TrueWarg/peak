use crate::task::Question;

#[derive(Debug, PartialEq, Eq)]
pub struct Plus {
    pub a: i32,
    pub b: i32,
}

impl Question for Plus {
    type Body = String;

    type Solution = i32;

    fn body(&self) -> Self::Body {
        return format!("{} + {}", self.a, self.b);
    }

    fn solution(&self) -> Self::Solution {
        return self.a + self.b;
    }
}
