mod benchmark;
mod debug;
mod jit;
mod optimize;

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum FlagTypes {
    None,
    Debug,
    Benchmark,
    Jit,
    Optimize, // There are no optimizations yet, this is for the near future
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
    // ToDo: Make Loop crash in an elegant way instead of calling the "panic" function of Rust
    // ToDo: This is regarding the whole implementation of Flags.

    fn get_flag(string: &str) -> Result<FlagTypes, String> {
        let flag_arguments: Vec<&str> = string.split('=').collect();

        if flag_arguments.len() > 2 {
            return Err(format!(
                "Found \"{}\" arguments, expected a max of one",
                flag_arguments.len() - 1
            ));
        }
        if flag_arguments.len() == 2 {
            return match flag_arguments[0] {
                "--debug" | "-d" => debug::debug_flag_with_param(flag_arguments[1]),
                "--benchmark" | "-b" => benchmark::benchmark_flag_with_param(flag_arguments[1]),
                "--jit" | "-j" => jit::jit_flag_with_param(flag_arguments[1]),
                "--optimize" | "-o" => optimize::optimize_flag_with_param(flag_arguments[1]),
                &_ => Err(format!(
                    "Found argument: \"{}\", which wasn't expected, or isn't valid in this context",
                    string
                )),
            };
        }
        return match flag_arguments[0] {
            "--debug" | "-d" => debug::debug_flag(),
            "--benchmark" | "-b" => benchmark::benchmark_flag(),
            "--jit" | "-j" => jit::jit_flag(),
            "--optimize" | "-o" => optimize::optimize_flag(),
            &_ => Err(format!(
                "Found argument: \"{}\", which wasn't expected, or isn't valid in this context",
                string
            )),
        };
    }

    pub fn parse_flags(&mut self, args: Vec<String>) -> i32 {
        let mut i: i32 = 0;

        for arg in args.clone() {
            let flag = Flags::get_flag(arg.as_str());
            match flag {
                Ok(e) => self.flags.push(e),
                Err(e) => {
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
