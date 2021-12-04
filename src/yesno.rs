use rand::Rng;
use clap::ArgMatches;

pub struct Yesno<'a> {
    answers: Vec<&'a str>,
    question: Option<&'a str>,
}

impl<'a> Yesno<'a> {
    pub fn new(question_option: &'a ArgMatches) -> Yesno<'a> {
        Yesno {
            answers: vec!["yes", "no"],
            question: question_option.value_of("question"),
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
