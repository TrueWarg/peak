use crate::task::Question;

#[derive(Debug, PartialEq, Eq)]
pub struct Sum {
    pub a: i32,
    pub b: i32,
}

pub struct Sub {
    pub a: i32,
    pub b: i32,
}

impl Question for Sum {
    fn body(&self) -> String {
        return format!("{} + {} = ?", self.a, self.b);
    }

    fn check(&self, answer: &String) -> bool {
        let answer: i32 = answer.trim().parse().expect("Input not an integer");
        let solution = self.a + self.b;
        answer == solution
    }
}

impl Question for Sub {
    fn body(&self) -> String {
        return format!("{} - {} = ?", self.a, self.b);
    }

    fn check(&self, answer: &String) -> bool {
        let answer: i32 = answer.trim().parse().expect("Input not an integer");
        let solution = self.a - self.b;
        answer == solution
    }
}
