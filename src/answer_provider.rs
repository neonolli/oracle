use rand::Rng;
use clap::ArgMatches;

pub trait AnswerProvider {
    fn new(matches: &ArgMatches) -> Self;

    fn answer() {

