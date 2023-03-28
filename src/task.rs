pub trait Question {
    fn body(&self) -> String;
    fn check(&self, answer: &String) -> bool;
}
