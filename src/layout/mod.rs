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
    Center,
    Vertical,
    Horizontal,
    Left,
    Right,
    Top,
    Bottom,
}

pub enum Offset {
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
    position: Position,
    size: Size,
    align: Align,
    offset: Offset,
    direction: Direction,
    padding: (usize, usize, usize, usize),
    spacing: (usize, usize),
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
