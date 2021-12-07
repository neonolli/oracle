use rand::Rng;
use clap::ArgMatches;

pub struct Coin<'a> {
    answers: Vec<&'a str>,
    question: Option<&'a str>,
    repeat: bool,
}

impl<'a> Coin<'a> {
    pub fn new(matches: &'a ArgMatches) -> Coin<'a> {
        Coin {
            answers: vec!["heads", "tails"],
            question: matches.value_of("question"),
            repeat: matches.is_present("repeat"),
        }
    }
    pub fn answer(&self) {
        let select = rand::thread_rng().gen_range(0..self.answers.len());
        match self.question {
            Some(q) => {
                if self.repeat {
                    println!("You asked: \"{}\"\n{}", q, self.answers[select]);
                } else {
                    println!("{}", self.answers[select]);
                }
            },
            None => println!("{}", self.answers[select]),
        }
    }
}
