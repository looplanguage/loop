use crate::lib::exception::flag;
use crate::lib::flags::FlagTypes;

pub fn jit_flag_with_param(parameter: &str) -> Result<FlagTypes, ()> {
    if parameter == "true" {
        return Ok(FlagTypes::Jit(Some(true)));
    }
    if parameter == "false" {
        return Ok(FlagTypes::Jit(Some(false)));
    }
    flag::throw_exception_unexpected_value(format!("-j | --jit = {}", parameter));
    Err(())
}

pub fn jit_flag() -> Result<FlagTypes, ()> {
    Ok(FlagTypes::Jit(Some(true)))
}
