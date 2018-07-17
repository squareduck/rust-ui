use self::format::format_with_indent;
use event::Handlers;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;
use template::SharedTemplate;
use types::CowString;

pub mod builder;
mod format;

pub type TextContent = CowString;
pub type Name = CowString;
pub type KeyValue = CowString;
pub type Key = Option<KeyValue>;
pub type Class = CowString;
pub type Classes = HashSet<Class>;
pub type AttrName = CowString;
pub type AttrValue = CowString;
pub type Attributes = HashMap<AttrName, AttrValue>;

pub type SharedText<A> = Arc<Text<A>>;
pub struct Text<A> {
    content: TextContent,
    handlers: Handlers<A>,
}

pub type SharedItem<A> = Arc<Item<A>>;
pub struct Item<A> {
    name: Name,
    key: Key,
    classes: Classes,
    attributes: Attributes,
    handlers: Handlers<A>,
}

pub type Children<S, M, A> = Vec<DynamicNode<S, M, A>>;

pub enum DynamicNode<S, M, A> {
    Text(SharedText<A>),
    Item(SharedItem<A>),
    Container(SharedItem<A>, Children<S, M, A>),
    Template(SharedTemplate<S, M, A>, Option<M>),
}

impl<S, M, A> DynamicNode<S, M, A> {
    pub fn render(&self, store: &S) -> StaticNode<A> {
        use self::DynamicNode::*;
        match self {
            Text(node) => StaticNode::Text(node.clone()),
            Item(node) => StaticNode::Item(node.clone()),
            Container(node, children) => StaticNode::Container(
                node.clone(),
                children.iter().map(|child| child.render(store)).collect(),
            ),
            Template(tpl, msg) => tpl.render(store, msg).render(store),
        }
    }
}

type StaticChildren<A> = Vec<StaticNode<A>>;

pub enum StaticNode<A> {
    Text(SharedText<A>),
    Item(SharedItem<A>),
    Container(SharedItem<A>, StaticChildren<A>),
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
