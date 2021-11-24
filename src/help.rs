use crate::answer::AnswerProvider;

pub struct HelpCommand<'a> {
    _answers: Vec<&'a str>,
}

impl<'a> AnswerProvider<'a> for HelpCommand<'a> {
    fn new() -> HelpCommand<'a> {
        HelpCommand {
            _answers: vec![""],
        }
    }

    fn answer(&self, _question: &str, _repeat: bool) {
        println!("Oracle v0.1.0");
        println!("Usage example: oracle [args] [question]");
        println!("Args: <optional>");
        println!("\tcoin -\tstandalone or followed by a question.");
        println!("\t\tAnswers: Heads or tails");
        println!("\tYesno -\tstandalone or followed by a question.");
        println!("\t\tAnswers: Yes or No");
        println!("\ttinyd6 -\tRolls checks for the TinyD6 rpg");
        println!("\t\tStandlone or followed by a third option.");
        println!("\t\t\tadv -\trolls check with advantage");
        println!("\t\t\tdis -\trolls check with disadvantage");
        println!("\thelp -\tprints this text.");
        println!("\t\tcan be called with -h, --help");
        println!("Question: <optional>");
        println!("");
        println!("At least one question or arguments must be supplied.");
        println!("set ORACLE_REPEAT_QUESTION environment variable");
        println!("to cause your questions to be repeated.");
        println!("===============================================================");

    }
}
