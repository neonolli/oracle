use std::env;

use oracle::answer::AnswerProvider;
use oracle::coin::Coin;
use oracle::crystal::Crystal;
use oracle::help::HelpCommand;
use oracle::tinyd6::TinyD6;
use oracle::tinyd6::TestType;
use oracle::yesno::Yesno;

struct Settings {
    coin: bool,
    yesno: bool,
    tinyd6: TestType,
    eight_ball: bool,
}

fn main() {
    let mut settings = Settings {
        coin: false,
        yesno: false,
        tinyd6: TestType::DEFAULT,
        eight_ball: true,
    };

    let mut question: &str = "";
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.is_empty() {
        eprintln!("How can I answer questions that haven't been asked?");
        std::process::exit(1);
    }

    args.iter().for_each(|arg| {
        match arg.as_str() {
            "coin" => {
                settings.coin = true;
                settings.eight_ball = false;
            },
            "yesno" => {
                settings.yesno = true;
                settings.eight_ball = false;
            },
            "tinyd6" => {
                settings.tinyd6 = TestType::NORM;
                settings.eight_ball = false;
            },
            "adv" => settings.tinyd6 = TestType::ADV,
            "dis" => settings.tinyd6 = TestType::DIS,
            "help" => HelpCommand::new().answer("", false),
            "-h" => HelpCommand::new().answer("", false),
            "--help" => HelpCommand::new().answer("", false),
            _ => question = arg,
        };
    });

    if settings.eight_ball {
        Crystal::new().answer(question, env::var("ORACLE_REPEAT_QUESTION").is_ok());
    } else if settings.coin {
        Coin::new().answer(question, env::var("ORACLE_REPEAT_QUESTION").is_ok());
    } else if settings.yesno {
        Yesno::new().answer(question, env::var("ORACLE_REPEAT_QUESTION").is_ok());
    } else {
        match settings.tinyd6 {
            TestType::NORM => TinyD6::new().roll(settings.tinyd6),
            TestType::DIS => TinyD6::new().roll(settings.tinyd6),
            TestType::ADV => TinyD6::new().roll(settings.tinyd6),
            _ => panic!("Not a TinyD6 roll type."),
        };
    }
}
