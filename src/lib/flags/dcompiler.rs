use crate::lib::flags::FlagTypes;

pub fn dcompiler_flag_with_param(parameter: &str) -> Result<FlagTypes, ()> {
    if parameter.is_empty() {
        return Ok(FlagTypes::DCompiler(Some(parameter.to_string())));
    }

    Ok(FlagTypes::DCompiler(Some(parameter.to_string())))
}

pub fn dcompiler_flag() -> Result<FlagTypes, ()> {
    Ok(FlagTypes::DCompiler(Some("".to_string())))
}
