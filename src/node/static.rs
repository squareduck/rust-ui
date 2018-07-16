use dynamic_node::{AttributeMap, ClassList, Key, Tag, TextContent};
use event::Handlers;

mod format;

pub enum StaticNode<'node, A>
where
    A: 'node,
{
    Container(Container<'node, A>),
    Item(Item<'node, A>),
    Text(Text<'node, A>),
}

impl<'node, A> StaticNode<'node, A> {
    pub fn container(
        name: &'node Tag,
        key: &'node Key,
        classes: &'node ClassList,
        attributes: &'node AttributeMap,
        handlers: &'node Handlers<A>,
        children: ChildList<'node, A>,
    ) -> StaticNode<'node, A> {
        StaticNode::Container(Container {
            name: name,
            key: key,
            classes: classes,
            attributes: attributes,
            handlers: handlers,
            children: children,
        })
    }

    pub fn item(
        name: &'node Tag,
        key: &'node Key,
        classes: &'node ClassList,
        attributes: &'node AttributeMap,
        handlers: &'node Handlers<A>,
    ) -> StaticNode<'node, A> {
        StaticNode::Item(Item {
            name: name,
            key: key,
            classes: classes,
            attributes: attributes,
            handlers: handlers,
        })
    }

    pub fn text(content: &'node TextContent, handlers: &'node Handlers<A>) -> StaticNode<'node, A> {
        StaticNode::Text(Text {
            content: content,
            handlers: handlers,
        })
    }
}

pub type ChildList<'node, A> = Vec<StaticNode<'node, A>>;

pub struct Container<'node, A>
where
    A: 'node,
{
    name: &'node Tag,
    key: &'node Key,
    classes: &'node ClassList,
    attributes: &'node AttributeMap,
    handlers: &'node Handlers<A>,
    children: ChildList<'node, A>,
}

pub struct Item<'node, A>
where
    A: 'node,
{
    name: &'node Tag,
    key: &'node Key,
    classes: &'node ClassList,
    attributes: &'node AttributeMap,
    handlers: &'node Handlers<A>,
}

pub struct Text<'node, A>
where
    A: 'node,
{
    content: &'node TextContent,
    handlers: &'node Handlers<A>,
}
