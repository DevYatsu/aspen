use super::value::AspenValue;

pub mod io;
pub mod math;

pub fn import_module<'a>(name: &'a str) -> Option<AspenValue<'a>> {
    match name {
        "io" => Some(io::module()),
        "math" => Some(math::module()),
        _ => None,
    }
}
