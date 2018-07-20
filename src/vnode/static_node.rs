use std::fmt;
use std::sync::Arc;

use super::{Item, Text};
use layout::ContainerLayout;

use super::format::format_with_indent;

pub type StaticText<A> = Arc<Text<A>>;

pub type StaticItem<A> = Arc<Item<A>>;

pub type StaticChildren<A> = Vec<StaticNode<A>>;
pub struct StaticContainer<A> {
    item: StaticItem<A>,
    children: StaticChildren<A>,
    layout: Arc<ContainerLayout>,
}

impl<A> StaticContainer<A> {
    pub fn item(&self) -> &StaticItem<A> {
        &self.item
    }

    pub fn children(&self) -> &StaticChildren<A> {
        &self.children
    }

    pub fn layout(&self) -> &Arc<ContainerLayout> {
        &self.layout
    }
}

pub enum StaticNode<A> {
    Text(StaticText<A>),
    Item(StaticItem<A>),
    Container(StaticContainer<A>),
}

impl<A> StaticNode<A> {
    pub fn new_text(text: StaticText<A>) -> Self {
        StaticNode::Text(text)
    }

    pub fn new_item(item: StaticItem<A>) -> Self {
        StaticNode::Item(item)
    }

    pub fn new_container(
        item: StaticItem<A>,
        children: StaticChildren<A>,
        layout: Arc<ContainerLayout>,
    ) -> Self {
        StaticNode::Container(StaticContainer {
            item,
            children,
            layout,
        })
    }
}

impl<A> fmt::Display for StaticNode<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_with_indent(f, None, self)
    }
}

impl<A> fmt::Debug for StaticNode<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_with_indent(f, Some(0), self)
    }
}
