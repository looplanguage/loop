use crate::lib::flags::FlagTypes;

pub fn jit_flag_with_param(parameter: &str) -> Result<FlagTypes, String> {
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

pub fn jit_flag() -> Result<FlagTypes, String> {
    Ok(FlagTypes::Jit)
}
