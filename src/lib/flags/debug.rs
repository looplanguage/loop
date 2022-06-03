use crate::lib::exception::flag;
use crate::lib::flags::FlagTypes;

pub fn debug_flag_with_param(parameter: &str) -> Result<FlagTypes, ()> {
    if parameter == "true" {
        return Ok(FlagTypes::Debug(Some(true)));
    }
    if parameter == "false" {
        return Ok(FlagTypes::Debug(Some(false)));
    }
    flag::throw_exception_unexpected_value(format!("-d | --debug = {}", parameter));
    Err(())
}

pub fn debug_flag() -> Result<FlagTypes, ()> {
    Ok(FlagTypes::Debug(Some(true)))
}
