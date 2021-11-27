use crate::answer::AnswerProvider;
use rand::Rng;

pub struct Coin<'a> {
    answers: Vec<&'a str>,
}

impl<'a> AnswerProvider<'a> for Coin<'a> {
    fn new() -> Coin<'a> {
        Coin {
            answers: vec!["heads", "tails"],
        }
    }
    fn answer(&self, question: &str, repeat_question: bool) {
        let select = rand::thread_rng().gen_range(0..self.answers.len());
        if repeat_question {
            println!("{} {}", question, self.answers[select]);
        } else {
            println!("{}", self.answers[select]);
        }
    }
}
