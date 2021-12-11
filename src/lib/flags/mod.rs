#[derive(PartialEq)]
pub enum FlagTypes {
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
    fn get_flag(string: &str) -> Result<FlagTypes, String> {
        match string {
            "--debug" | "-d" => Ok(FlagTypes::Debug),
            "--benchmark" | "-b" => Ok(FlagTypes::Benchmark),
            "--jit" | "-j" => Ok(FlagTypes::Jit),
            &_ => Err(format!(
                "Found argument: \"{}\", which wasn't expected, or isn't valid in this context",
                string
            )),
        }
    }

    pub fn parse_flags(&mut self, args: Vec<String>) -> i32 {
        let mut i: i32 = 0;

        for arg in args.clone() {
            let flag = Flags::get_flag(arg.as_str());
            match flag {
                Ok(e) => self.flags.push(e),
                Err(e) => {
                    // ToDo: Make Loop crash in an elegant way instead of calling the "panic" function of Rust
                    panic!("{}", e);
                }
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
