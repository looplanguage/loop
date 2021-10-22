#[derive(PartialEq)]
pub enum FlagTypes {
    None,
    Debug,
    Benchmark,
    Jit,
}

pub fn build_flags() -> Flags {
    Flags {
        flags: vec![],
        file: None,
    }
}

pub struct Flags {
    pub flags: Vec<FlagTypes>,
    pub file: Option<String>,
}

impl Flags {
    fn get_flag(string: &str) -> FlagTypes {
        match string {
            "--debug" | "-d" => FlagTypes::Debug,
            "--benchmark" | "-b" => FlagTypes::Benchmark,
            "--jit" | "-j" => FlagTypes::Jit,
            &_ => FlagTypes::None,
        }
    }

    pub fn parse_flags(&mut self, args: Vec<String>) -> i32 {
        let mut i: i32 = 0;

        for arg in args.clone() {
            let flag = Flags::get_flag(arg.as_str());
            if flag != FlagTypes::None {
                self.flags.push(flag)
            } else {
                break;
            }

            i += 1;
        }

        if args.len() as i32 > i {
            self.file = Option::from(args[i as usize].clone());
        }

        i
    }

    pub fn contains(&self, flag: FlagTypes) -> bool {
        self.flags.contains(&flag)
    }
}
