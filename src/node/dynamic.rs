use super::*;
use component::Component;
use event::Handlers;
use std::sync::Arc;

pub struct ComponentRef<'cpt, S, M, A>
where
    S: 'cpt,
    M: 'cpt,
    A: 'cpt,
{
    component: Arc<Component<'cpt, S, M, A>>,
    message: Option<M>,
}

pub struct Container<'cpt, S, M, A>
where
    S: 'cpt,
    M: 'cpt,
    A: 'cpt,
{
    name: Arc<Tag>,
    key: Arc<Key>,
    classes: Arc<ClassList>,
    attributes: Arc<AttributeMap>,
    handlers: Arc<Handlers<A>>,
    children: ChildList<'cpt, S, M, A>,
}

pub struct Item<A> {
    name: Arc<Tag>,
    key: Arc<Key>,
    classes: Arc<ClassList>,
    attributes: Arc<AttributeMap>,
    handlers: Arc<Handlers<A>>,
}

pub struct Text<A> {
    content: Arc<TextContent>,
    handlers: Arc<Handlers<A>>,
}

/// A node with possible dynamic segments.
///
/// Type parameters:
/// * `S`: Store value
/// * `M`: Message enum
/// * `A`: Action enum
///
pub enum DynamicNode<'cpt, S, M, A>
where
    S: 'cpt,
    M: 'cpt,
    A: 'cpt,
{
    Component(ComponentRef<'cpt, S, M, A>),
    Container(Container<'cpt, S, M, A>),
    Item(Item<A>),
    Text(Text<A>),
}

impl<'cpt, S, M, A> DynamicNode<'cpt, S, M, A> {
    pub fn component(
        component: Arc<Component<'cpt, S, M, A>>,
        message: Option<M>,
    ) -> DynamicNode<'cpt, S, M, A> {
        DynamicNode::Component(ComponentRef {
            component: component,
            message: message,
        })
    }

    pub fn container(
        name: Tag,
        key: Key,
        classes: ClassList,
        attributes: AttributeMap,
        handlers: Handlers<A>,
        children: ChildList<'cpt, S, M, A>,
    ) -> DynamicNode<'cpt, S, M, A> {
        DynamicNode::Container(Container {
            name: Arc::new(name),
            key: Arc::new(key),
            classes: Arc::new(classes),
            attributes: Arc::new(attributes),
            handlers: Arc::new(handlers),
            children: children,
        })
    }

    pub fn item(
        name: Tag,
        key: Key,
        classes: ClassList,
        attributes: AttributeMap,
        handlers: Handlers<A>,
    ) -> DynamicNode<'cpt, S, M, A> {
        DynamicNode::Item(Item {
            name: Arc::new(name),
            key: Arc::new(key),
            classes: Arc::new(classes),
            attributes: Arc::new(attributes),
            handlers: Arc::new(handlers),
        })
    }

    pub fn text(content: TextContent, handlers: Handlers<A>) -> DynamicNode<'cpt, S, M, A> {
        DynamicNode::Text(Text {
            content: Arc::new(content),
            handlers: Arc::new(handlers),
        })
    }
    pub fn render(&self, store: &S) -> CachedNode<A> {
        match self {
            DynamicNode::Component(node) => node.component.render(store, &node.message),
            DynamicNode::Container(node) => CachedNode::container(
                node.name.clone(),
                node.key.clone(),
                node.classes.clone(),
                node.attributes.clone(),
                node.handlers.clone(),
                node.children
                    .iter()
                    .map(|child| child.render(&store))
                    .collect(),
            ),
            DynamicNode::Item(node) => CachedNode::item(
                node.name.clone(),
                node.key.clone(),
                node.classes.clone(),
                node.attributes.clone(),
                node.handlers.clone(),
            ),
            DynamicNode::Text(node) => {
                CachedNode::text(node.content.clone(), node.handlers.clone())
            }
        }
    }
}
