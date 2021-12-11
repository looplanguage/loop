use crate::lib::flags::FlagTypes;

pub fn benchmark_flag_with_param(parameter: &str) -> Result<FlagTypes, String> {
    if parameter == "true" {
        return Ok(FlagTypes::Benchmark);
    }
    if parameter == "false" {
        return Ok(FlagTypes::None);
    }
    return Err(format!(
        "Found parameter: \"{}\", which wasn't expected, or isn't valid in this context",
        parameter
    ));
}

pub fn benchmark_flag() -> Result<FlagTypes, String> {
    Ok(FlagTypes::Benchmark)
}
