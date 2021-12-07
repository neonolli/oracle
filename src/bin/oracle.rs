use clap::{App, Arg, ArgMatches};

use oracle::tinyd6::TinyD6;
use oracle::coin::Coin;
use oracle::crystal::Crystal;
use oracle::yesno::Yesno;

fn main() {
    let matches = App::new(clap::crate_name!())
        .author(clap::crate_authors!("\n"))
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .arg(Arg::new("question")
             .about("Question to be answered.")
             .global(true))
        .arg(Arg::new("repeat")
             .about("Repeat the question in the output")
             .long("repeat")
             .short('r')
             .global(true))
        .subcommand(
            App::new("tinyd6")
            .about("Rolls tests for tinyd6")
            .arg(Arg::new("adv")
                 .about("Rolls test with advantage.")
                 .possible_value("adv")
                 .possible_value("dis")))
        .subcommand(
            App::new("coin")
            .about("Flips a coin."))
        .subcommand(
            App::new("yesno")
            .about("Answers with a yes or a no."))
        .get_matches();

    match matches.subcommand() {
        Some(("tinyd6", subcmd)) => TinyD6::new(&subcmd).roll(),
        Some(("coin", subcmd)) => Coin::new(&subcmd).answer(),
        Some(("yesno", subcmd)) => Yesno::new(&subcmd).answer(),
        Some((&_, subcmd)) => run_default(&subcmd),
        None => run_default(&matches),
    }
}

fn run_default(matches: &ArgMatches) {
    if matches.is_present("question") {
        Crystal::new(matches).answer();
    } else {
        eprintln!("How can I answer a question that you haven't asked?");
        std::process::exit(1);
    }
}
