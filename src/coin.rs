use rand::Rng;
use clap::ArgMatches;

pub struct Coin<'a> {
    answers: Vec<&'a str>,
    question: Option<&'a str>,
}

impl<'a> Coin<'a> {
    pub fn new(question_optional: &'a ArgMatches) -> Coin<'a> {
        Coin {
            answers: vec!["heads", "tails"],
            question: question_optional.value_of("question"),
        }
    }
    pub fn answer(&self) {
        let select = rand::thread_rng().gen_range(0..self.answers.len());
        match self.question {
            Some(q) => println!("{}\n{}", q, self.answers[select]),
            None => println!("{}", self.answers[select]),
        }
    }
}
