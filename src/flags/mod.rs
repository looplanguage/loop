#[derive(PartialEq)]
pub enum FlagTypes {
    None,
    Debug,
}

pub fn build_flags() -> Flags {
    Flags { flags: vec![] }
}

pub struct Flags {
    pub flags: Vec<FlagTypes>,
}

impl Flags {
    fn get_flag(string: &str) -> FlagTypes {
        match string {
            "--debug" | "-d" => FlagTypes::Debug,
            &_ => FlagTypes::None,
        }
    }

    pub fn parse_flags(&mut self, args: Vec<String>) {
        for arg in args {
            let flag = Flags::get_flag(arg.as_str());
            if flag != FlagTypes::None {
                self.flags.push(flag)
            }
        }
    }

    pub fn contains(self, flag: FlagTypes) -> bool {
        self.flags.contains(&flag)
    }
}
