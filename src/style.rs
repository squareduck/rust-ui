use types::CowString;

pub type Color = (u8, u8, u8, u8);

pub struct Border {
    color: Color,
    width: usize,
}

pub struct Font {
    name: CowString,
    size: usize,
}

pub struct Style {
    color: Color,
    text_color: Color,
    font: Font,
    border: Border,
}
