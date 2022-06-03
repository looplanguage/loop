use crate::lib::exception::flag;
use crate::lib::flags::FlagTypes;

pub fn optimize_flag_with_param(parameter: &str) -> Result<FlagTypes, ()> {
    if parameter == "true" {
        return Ok(FlagTypes::Optimize(Some(true)));
    }
    if parameter == "false" {
        return Ok(FlagTypes::Optimize(Some(false)));
    }
    flag::throw_exception_unexpected_value(format!("-o | --optimize = {}", parameter));
    Err(())
}

pub fn optimize_flag() -> Result<FlagTypes, ()> {
    Ok(FlagTypes::Optimize(Some(true)))
}
