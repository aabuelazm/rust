use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Debug, PartialEq, IntoEnumIterator, IntEnum, Eq, Copy, Clone)]
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
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    match value {
        0 => "Black".to_owned(),
        1 => "Brown".to_owned(),
        2 => "Red".to_owned(),
        3 => "Orange".to_owned(),
        4 => "Yellow".to_owned(),
        5 => "Green".to_owned(),
        6 => "Blue".to_owned(),
        7 => "Violet".to_owned(),
        8 => "Grey".to_owned(),
        9 => "White".to_owned(),
        _ => "value out of range".to_owned(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut ordered: Vec<ResistorColor> = Vec::new();

    for n in 0..10 {
        ordered.insert(n, ResistorColor::from_int(n).expect("error"));
    }

    ordered
}
