use crate::lib::flags::FlagTypes;

pub fn lua_flag() -> Result<FlagTypes, ()> {
    Ok(FlagTypes::Lua(Some(true)))
}
