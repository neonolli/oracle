use rand::Rng;

pub struct Crystal<'a> {
    answers: Vec<&'a str>,
    question: &'a str,
}

impl<'a> Crystal<'a> {
    pub fn new(question: &'a str) -> Crystal<'a> {
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

        Crystal { answers, question }
    }

    pub fn answer(&self) {
        let selection = rand::thread_rng().gen_range(0..self.answers.len());
        println!("{}\n{}", self.question, self.answers[selection]);
    }
}
