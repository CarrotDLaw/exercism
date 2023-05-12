use enum_iterator::*;
use int_enum::*;

#[derive(Debug, PartialEq, IntoEnumIterator, IntEnum, Clone, Copy,Eq, PartialOrd, Ord)]
#[repr(usize)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(resistor) => format!("{:?}", resistor),
        Err(_) => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut result: Vec<ResistorColor> = ResistorColor::into_enum_iter().collect();
    result.sort();
    result
}
