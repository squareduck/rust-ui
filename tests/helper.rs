extern crate cinnabar;

use cinnabar::vnode::TextContent;
use cinnabar::Builder;

pub struct Store {
    pub points: usize,
}

pub enum Message {}
pub enum Action {
    None,
    Increment,
}

pub fn panel() -> Builder<Store, Message, Action> {
    Builder::container("panel")
}

pub fn button() -> Builder<Store, Message, Action> {
    Builder::container("button")
}

pub fn text<T: Into<TextContent>>(content: T) -> Builder<Store, Message, Action> {
    Builder::text(content.into())
}
