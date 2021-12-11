use crate::lib::flags::FlagTypes;

pub(crate) fn optimize_flag(parameter: &str) -> Result<FlagTypes, String> {
    if parameter == "true" {
        return Ok(FlagTypes::Jit);
    }
    if parameter == "false" {
        return Ok(FlagTypes::None);
    }
    return Err(format!(
        "Found parameter: \"{}\", which wasn't expected, or isn't valid in this context",
        parameter
    ));
}
