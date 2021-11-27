pub trait AnswerProvider<'a> {
    fn new() -> Self;
    fn answer(&self, question: &str, repeat_question: bool);
}
