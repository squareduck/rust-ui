use event::Handlers;
use static_node::StaticNode;
use std::collections::{HashMap, HashSet};
use types::CowString;

pub mod builder;

pub type Template<'node, S, M, A> = fn(&S, &Option<M>) -> StaticNode<'node, A>;

/// A node with possible dynamic segments.
///
/// Type parameters:
/// * `S`: Store value
/// * `M`: Message enum
/// * `A`: Action enum
///
pub enum DynamicNode<'node, S, M, A>
where
    A: 'node,
{
    Component(Template<'node, S, M, A>, Option<M>),
    Container(Container<'node, S, M, A>),
    Item(Item<A>),
    Text(Text<A>),
}

impl<'node, S, M, A> DynamicNode<'node, S, M, A> {
    pub fn render(&self, store: &S) -> StaticNode<A> {
        match self {
            DynamicNode::Component(template, message) => template(store, message),
            DynamicNode::Container(node) => StaticNode::container(
                &node.name,
                &node.key,
                &node.classes,
                &node.attributes,
                &node.handlers,
                node.children
                    .iter()
                    .map(|child| child.render(&store))
                    .collect(),
            ),
            DynamicNode::Item(node) => StaticNode::item(
                &node.name,
                &node.key,
                &node.classes,
                &node.attributes,
                &node.handlers,
            ),
            DynamicNode::Text(node) => StaticNode::text(&node.content, &node.handlers),
        }
    }
}

pub type Tag = CowString;
pub type KeyValue = CowString;
pub type Key = Option<CowString>;
pub type TextContent = CowString;
pub type Class = CowString;
pub type ClassList = HashSet<Class>;
pub type AttributeName = CowString;
pub type AttributeValue = CowString;
pub type AttributeMap = HashMap<AttributeName, AttributeValue>;
pub type ChildList<'node, S, M, A> = Vec<DynamicNode<'node, S, M, A>>;

pub struct Container<'node, S, M, A>
where
    A: 'node,
{
    name: Tag,
    key: Key,
    classes: ClassList,
    attributes: AttributeMap,
    handlers: Handlers<A>,
    children: ChildList<'node, S, M, A>,
}

pub struct Item<A> {
    name: Tag,
    key: Key,
    classes: ClassList,
    attributes: AttributeMap,
    handlers: Handlers<A>,
}

pub struct Text<A> {
    content: TextContent,
    handlers: Handlers<A>,
}
