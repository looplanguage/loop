use crate::lib::flags::FlagTypes;

pub(crate) fn benchmark_flag(parameter: &str) -> Result<FlagTypes, String> {
    if parameter == "true" {
        return Ok(FlagTypes::Benchmark);
    }
    if parameter == "false" {
        return Ok(FlagTypes::Benchmark);
    }
    return Err(format!(
        "Found parameter: \"{}\", which wasn't expected, or isn't valid in this context",
        parameter
    ));
}
