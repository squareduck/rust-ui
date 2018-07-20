use vnode::IdValue;

//
// # Common
//
pub enum Position {
    Auto,
    Top(IdValue),
    Bottom(IdValue),
    Left(IdValue),
    Right(IdValue),
    Constant(usize, usize),
}

pub enum Size {
    Fill,
    FillRatio(usize),
    Wrap,
    Constant(usize, usize),
    ConstantWidth(usize),
    ConstantHeight(usize),
}

pub enum Align {
    None,
    Center,
    Left,
    Right,
    Top,
    Bottom,
}

pub enum Offset {
    None,
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

//
// # Container
//
pub enum Direction {
    Row,
    Column,
}

pub struct ContainerLayout {
    direction: Direction,
    padding: (usize, usize, usize, usize),
    spacing: (usize, usize),
}

impl Default for ContainerLayout {
    fn default() -> ContainerLayout {
        ContainerLayout {
            direction: Direction::Row,
            padding: (0, 0, 0, 0),
            spacing: (0, 0),
        }
    }
}

//
// # Item
//
pub struct ItemLayout {
    position: Position,
    align: Align,
    offset: Offset,
    size: Size,
}

impl Default for ItemLayout {
    fn default() -> ItemLayout {
        ItemLayout {
            position: Position::Auto,
            align: Align::None,
            offset: Offset::None,
            size: Size::Fill,
        }
    }
}
