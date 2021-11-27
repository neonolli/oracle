use rand::rngs::ThreadRng;
use rand::Rng;

pub enum TestType {
    DIS,
    NORM,
    ADV,
    DEFAULT,
}

pub struct TinyD6 {
    rand: ThreadRng,
}

impl Default for TinyD6 {
    fn default() -> Self {
        Self::new()
    }
}

impl TinyD6 {
    pub fn new() -> TinyD6 {
        let rand = rand::thread_rng();
        TinyD6 { rand }
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

    pub fn roll(&mut self, roll_type: TestType) {
        match roll_type {
            TestType::ADV => self.get_roll(3),
            TestType::DIS => self.get_roll(1),
            TestType::NORM => self.get_roll(2),
            TestType::DEFAULT => self.get_roll(2),
        };
    }
}
