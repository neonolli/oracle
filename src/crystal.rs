use rand::Rng;
use clap::ArgMatches;

pub struct Crystal<'a> {
    answers: Vec<&'a str>,
    question: Option<&'a str>,
    repeat: bool,
}

impl<'a> Crystal<'a> {
    pub fn new(matches: &'a ArgMatches) -> Crystal<'a> {
        let answers = vec![
            "It is certain.",
            "It is decidedly so.",
            "Without a doubt.",
            "Yes, definitely.",
            "You may rely on it.",
            "As I see it, yes.",
            "Most likely.",
            "Outlook good.",
            "Yes.",
            "Signs point to yes.",
            "Reply hazy, try again.",
            "Ask again later.",
            "Better not tell you now.",
            "Concentrate and ask again.",
            "Don't count on it.",
            "My reply is no.",
            "My sources say no.",
            "Outlook not so good.",
            "Very doubtful.",
        ];

        Crystal {
            answers,
            question: matches.value_of("question"),
            repeat: matches.is_present("repeat"),
        }
    }

    pub fn answer(&self) {
        let selection = rand::thread_rng().gen_range(0..self.answers.len());
        match self.question {
            Some(q) => {
                if self.repeat {
                    println!("You asked \"{}\"\n{}", q, self.answers[selection]);
                } else {
                    println!("{}", self.answers[selection]);
                }
            },
            None => println!("{}", self.answers[selection]),
        }
    }
}
