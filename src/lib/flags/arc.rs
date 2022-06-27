use crate::lib::flags::FlagTypes;

pub fn arc_flag() -> Result<FlagTypes, ()> {
    Ok(FlagTypes::Arc(Some(true)))
}
