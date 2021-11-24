use rand::Rng;
use crate::answer::AnswerProvider;

pub struct Yesno<'a> {
    answers: Vec<&'a str>,
} 

impl<'a> AnswerProvider<'a> for Yesno<'a> {
    fn new() -> Yesno<'a> {
        Yesno {
            answers: vec!["yes","no"],
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
