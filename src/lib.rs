pub mod answer;
pub mod coin;
pub mod crystal;
pub mod help;
pub mod tinyd6;
pub mod yesno;

pub struct Settings {
    pub coin: bool,
    pub yesno: bool,
    pub tinyd6: crate::tinyd6::TestType,
    pub eight_ball: bool,
}

//#cfg(test)]
//mod test;
