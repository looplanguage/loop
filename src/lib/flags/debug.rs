use crate::lib::flags::FlagTypes;

pub(crate) fn debug_flag(parameter: &str) -> Result<FlagTypes, String> {
    if parameter == "true" {
        return Ok(FlagTypes::Debug);
    }
    if parameter == "false" {
        return Ok(FlagTypes::Debug);
    }
    return Err(format!(
        "Found parameter: \"{}\", which wasn't expected, or isn't valid in this context",
        parameter
    ));
}
