use rand::Rng;
use clap::ArgMatches;

pub struct Yesno<'a> {
    answers: Vec<&'a str>,
    question: Option<&'a str>,
    repeat: bool,
}

impl<'a> Yesno<'a> {
    pub fn new(matches: &'a ArgMatches) -> Yesno<'a> {
        Yesno {
            answers: vec!["yes", "no"],
            question: matches.value_of("question"),
            repeat: matches.is_present("repeat"),
        }
    }
    pub fn answer(&self) {
        let select = rand::thread_rng().gen_range(0..self.answers.len());
        match self.question {
            Some(q) => {
                if self.repeat {
                    println!("{}\n{}", q, self.answers[select]);
                } else {
                    println!("{}", self.answers[select]);
                }
            },
            None => println!("{}", self.answers[select]),
        }
    }
}
