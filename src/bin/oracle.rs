use clap::{App, Arg};

use oracle::tinyd6::TinyD6;
use oracle::coin::Coin;
use oracle::crystal::Crystal;
use oracle::yesno::Yesno;

fn main() {
    let matches = App::new(clap::crate_name!())
        .author(clap::crate_authors!("\n"))
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .subcommand(
            App::new("tinyd6")
            .about("Rolls tests for tinyd6")
            .arg(Arg::new("adv")
                 .about("Rolls test with advantage.")
                 .possible_value("adv")
                 .possible_value("dis")))
        .subcommand(
            App::new("coin")
            .about("Flips a coin.")
            .arg(Arg::new("question")
                 .about("Question to be answered.")))
        .subcommand(
            App::new("yesno")
            .about("Answers with a yes or a no.")
            .arg(Arg::new("question").
                 about("Question to be answered.")))
        .arg(Arg::new("question")
             .about("Question to be answered."))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("tinyd6") {
        TinyD6::new(matches).roll();
    } else if let Some(matches) = matches.subcommand_matches("coin") {
        Coin::new(matches).answer();
    }  else if let Some(matches) = matches.subcommand_matches("yesno") {
        Yesno::new(matches).answer();
    } else {
        // See if we a have question at the very least and if not, warn user.
        if matches.is_present("question") {
            let question = matches.value_of("question").unwrap();
            Crystal::new(question).answer();
        } else {
            eprintln!("How can I answer a question that you haven't asked?");
            std::process::exit(1);
        }
    }
}
