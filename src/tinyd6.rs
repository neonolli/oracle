use rand::Rng;
use rand::rngs::ThreadRng;

pub struct TinyD6 {
    rand: ThreadRng,
}

impl TinyD6 {
    pub fn new() -> TinyD6 {
        let rand = rand::thread_rng();
        TinyD6 {
            rand
        }
    }

    fn get_rolls(&mut self, number_rolls: u8) -> Vec<i32> {
        let mut response = vec![];
        let mut i = 0;
        while i < number_rolls {
            response.push(self.rand.gen_range(1..7));
            i += 1;
        }
        return response;
    }

    fn is_success(&mut self, rolls: &Vec<i32>) -> bool {
        let mut success = false;
        for roll in rolls {
            if *roll >= 5 {
                success = true;
            }
        }
        return success;
    }

    fn is_critical(&mut self, rolls: &Vec<i32>) -> bool {
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
        return critical;
    }

    pub fn roll(&mut self) {
        let roll = self.get_rolls(2);
        let mut output = format!("[{},{}] - ", roll[0], roll[1]);
        let success = self.is_success(&roll);
        let critical = self.is_critical(&roll);
        if success {
            if critical {
                output.push_str("Critical Success!");
            } else {
                output.push_str("Success!");
            }
        } else {
            if critical {
                output.push_str("Critical Failure!");
            } else {
                output.push_str("Fail!");
            }
        }
        println!("{}", output);
    }

    pub fn roll_adv(&mut self) {
        let roll = self.get_rolls(3);
        let mut output = format!("[{},{},{}] - ",
                                 roll[0],
                                 roll[1],
                                 roll[2]);
        let success = self.is_success(&roll);
        let critical = self.is_critical(&roll);
        if success {
            if critical {
                output.push_str("Critical Success!");
            } else {
                output.push_str("Success!");
            }
        } else {
            if critical {
                output.push_str("Critical Failure!");
            } else {
                output.push_str("Fail!");
            }
        }
        println!("{}", output);
    }
    pub fn roll_dis(&mut self) {
        let roll = self.get_rolls(1);
        let mut output = format!("[{}] - ", roll[0]);
        let success = self.is_success(&roll);
        let critical = self.is_critical(&roll);
        if success {
            if critical {
                output.push_str("Critical Success!");
            } else {
                output.push_str("Success!");
            }
        } else {
            if critical {
                output.push_str("Critical Failure!");
            } else {
                output.push_str("Fail!");
            }
        }
        println!("{}", output);
    }
}
