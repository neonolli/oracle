use std::env;

use oracle::coin::Coin;
use oracle::yesno::Yesno;
use oracle::crystal::Crystal;
use oracle::answer::AnswerProvider;
use oracle::tinyd6::TinyD6;
use oracle::help::HelpCommand;

struct Settings {
    coin: bool,
    yesno: bool,
    tinyd6: bool,
    td6_adv: bool,
    td6_dis: bool,
    eight_ball: bool,
}

fn main() {
    let mut settings = Settings {
        coin: false,
        yesno: false,
        tinyd6: false,
        td6_adv: false,
        td6_dis: false,
        eight_ball: true,
    };

    let mut question: &str = "";
    let mut args: Vec<String> = env::args().collect();
    
    let _program_name: String = args[0].clone();
    
    args.remove(0);

    if args.len() < 1 {
        eprintln!("How can I answer questions that haven't been asked?");
        std::process::exit(1);
    }

    args.iter().for_each(|arg| {
        if arg == "coin" {
            if !settings.yesno && !settings.tinyd6 {
                settings.eight_ball = false;
                settings.coin = true;
            } else {
                eprintln!("coin must be used by itself.");
                std::process::exit(1);
            }
        } else if arg == "yesno" {
            if !settings.coin && !settings.tinyd6 {
                settings.eight_ball = false;
                settings.yesno = true;
            } else {
                eprintln!("yesno must be used by itself.");
                std::process::exit(1);
            }
        } else if arg == "tinyd6" {
            if !settings.coin && !settings.yesno {
                settings.eight_ball = false;
                settings.tinyd6 = true
            } else {
                eprintln!("tinyd6 must be used by itself.");
                std::process::exit(1);
            }
        } else if arg == "adv" {
            if settings.tinyd6 {
                settings.td6_dis = false;
                settings.td6_adv = true;
            } else {
                eprintln!("adv can not be set if not using tinyd6 mode");
                std::process::exit(1);
            }
        } else if arg == "dis" {
            if settings.tinyd6 {
                settings.td6_adv = false;
                settings.td6_dis = true;
            } else {
                eprintln!("dis can not be set if not using tinyd6 mode");
                std::process::exit(1);
            }
        }  else if arg == "-h" || arg == "--help" || arg == "help" {
            HelpCommand::new().answer("",false);
            std::process::exit(0);
        } else {
            question = arg;
        }
    });
    
    if !settings.tinyd6 {
        if question != "" {
            if settings.eight_ball {
                Crystal::new().answer(question, env::var("ORACLE_REPEAT_QUESTION").is_ok());
            } else if settings.coin {
                Coin::new().answer(question, env::var("ORACLE_REPEAT_QUESTION").is_ok());
            } else if settings.yesno {
                Yesno::new().answer(question, env::var("ORACLE_REPEAT_QUESTION").is_ok());
            }
        } else {
            if settings.coin {
                Coin::new().answer("",false);
            } else if settings.yesno {
                Yesno::new().answer("",false);
            }
        }
    } else {
        let mut td6 = TinyD6::new();
        if !settings.td6_dis && !settings.td6_adv {
            td6.roll();
        } else if settings.td6_dis && !settings.td6_adv {
            td6.roll_dis();
        } else {
            td6.roll_adv();
        }
    }
}
