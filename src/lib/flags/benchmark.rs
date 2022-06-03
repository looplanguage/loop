use crate::lib::exception::flag;
use crate::lib::flags::FlagTypes;

pub fn benchmark_flag_with_param(parameter: &str) -> Result<FlagTypes, ()> {
    if parameter == "true" {
        return Ok(FlagTypes::Benchmark(Some(true)));
    }
    if parameter == "false" {
        return Ok(FlagTypes::Benchmark(Some(false)));
    }
    flag::throw_exception_unexpected_value(format!("-b | --benchmark = {}", parameter));
    Err(())
}

pub fn benchmark_flag() -> Result<FlagTypes, ()> {
    Ok(FlagTypes::Benchmark(Some(true)))
}
