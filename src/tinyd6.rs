use rand::rngs::ThreadRng;
use rand::Rng;
use clap::ArgMatches;

enum TestType {
    Dis,
    Norm,
    Adv,
}

pub struct TinyD6 {
    rand: ThreadRng,
    roll_type: TestType,
}


impl TinyD6 {
    pub fn new(matches: &ArgMatches) -> TinyD6 {
        let rand = rand::thread_rng();
        let mut roll_type = TestType::Norm;
        if matches.is_present("adv") {
            if matches.value_of("adv").unwrap().eq_ignore_ascii_case("adv") {
                roll_type = TestType::Adv;
            } else if matches.value_of("adv").unwrap().eq_ignore_ascii_case("dis") {
                roll_type = TestType::Dis;
            }
        }
        TinyD6 { rand, roll_type }
    }

    fn get_roll(&mut self, number_rolls: u8) {
        let mut response = vec![];
        let mut i = 0;
        while i < number_rolls {
            response.push(self.rand.gen_range(1..7));
            i += 1;
        }
        println!("{}", self.get_output(&response));
    }

    fn is_success(&mut self, rolls: &[i32]) -> bool {
        let mut success = false;
        for roll in rolls {
            if *roll >= 5 {
                success = true;
            }
        }
        success
    }

    fn is_critical(&mut self, rolls: &[i32]) -> bool {
        let mut critical = false;
        let mut count: i32 = 0;
        for roll in rolls {
            if *roll == 6 {
                count += 1;
            } else if *roll == 1 {
                count -= 1;
            }
        }
        if count.abs() == rolls.len().try_into().unwrap() {
            critical = true;
        }
        critical
    }

    fn get_output(&mut self, rolls: &[i32]) -> String {
        let mut output = format!("{:?} - ", rolls);
        let success = self.is_success(rolls);
        let critical = self.is_critical(rolls);
        if success && critical {
            output.push_str("Critical Success!");
        } else if success && !critical {
            output.push_str("Success!");
        } else if !success && critical {
            output.push_str("Critical Fail!");
        } else {
            output.push_str("Fail!");
        }
        output
    }

    pub fn roll(&mut self) {
        match self.roll_type {
            TestType::Adv => self.get_roll(3),
            TestType::Dis => self.get_roll(1),
            TestType::Norm => self.get_roll(2),
        };
    }
}
