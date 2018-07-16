use super::*;

use component::Component;
use event::ClickHandler;
use event::Handlers;
use node::dynamic::{ComponentRef, Container, Item, Text};
use std::sync::Arc;

// TODO: Stricter API for nodes?
// Have distinct enum variant for each possible node type.
// Each variant describes standard attributes it supports.
//
// Example:
// Button
// Panel
// Label
//
// Or have a few general categories such as:
// Area
// Layout
// Control
// Input

pub struct ComponentBuilder<'cpt, S, M, A>
where
    S: 'cpt,
    M: 'cpt,
    A: 'cpt,
{
    component: Arc<Component<'cpt, S, M, A>>,
    message: Option<M>,
}

pub struct ContainerBuilder<'cpt, S, M, A>
where
    S: 'cpt,
    M: 'cpt,
    A: 'cpt,
{
    name: Tag,
    key: Key,
    classes: ClassList,
    attributes: AttributeMap,
    handlers: Handlers<A>,
    children: ChildList<'cpt, S, M, A>,
}

pub struct ItemBuilder<A> {
    name: Tag,
    key: Key,
    classes: ClassList,
    attributes: AttributeMap,
    handlers: Handlers<A>,
}

pub struct TextBuilder<A> {
    content: TextContent,
    handlers: Handlers<A>,
}

pub enum Builder<'cpt, S, M, A>
where
    S: 'cpt,
    M: 'cpt,
    A: 'cpt,
{
    Component(ComponentBuilder<'cpt, S, M, A>),
    Container(ContainerBuilder<'cpt, S, M, A>),
    Item(ItemBuilder<A>),
    Text(TextBuilder<A>),
}

impl<'cpt, S, M, A> Builder<'cpt, S, M, A>
where
    S: 'cpt,
    M: 'cpt,
    A: 'cpt,
{
    pub fn component(component_ref: Arc<Component<'cpt, S, M, A>>) -> Self {
        Builder::Component(ComponentBuilder {
            component: component_ref,
            message: None,
        })
    }

    pub fn container<T: Into<Tag>>(name: T) -> Self {
        Builder::Container(ContainerBuilder {
            name: name.into(),
            key: None,
            classes: ClassList::new(),
            attributes: AttributeMap::new(),
            handlers: Handlers::new(),
            children: ChildList::new(),
        })
    }

    pub fn item<T: Into<Tag>>(name: T) -> Self {
        Builder::Item(ItemBuilder {
            name: name.into(),
            key: None,
            classes: ClassList::new(),
            attributes: AttributeMap::new(),
            handlers: Handlers::new(),
        })
    }

    pub fn text<T: Into<TextContent>>(content: T) -> Self {
        Builder::Text(TextBuilder {
            content: content.into(),
            handlers: Handlers::new(),
        })
    }

    pub fn key<T: Into<KeyValue>>(mut self, key: T) -> Self {
        match self {
            Builder::Container(ref mut container) => {
                container.key = Some(key.into());
            }
            Builder::Item(ref mut item) => item.key = Some(key.into()),
            _ => panic!("Only Container and Item can have children."),
        }
        self
    }

    pub fn class<T: Into<Class>>(mut self, class: T) -> Self {
        match self {
            Builder::Container(ref mut container) => {
                container.classes.insert(class.into());
            }
            Builder::Item(ref mut item) => {
                item.classes.insert(class.into());
            }
            _ => panic!("Only Container and Item can have classes."),
        }

        self
    }

    pub fn classes<T: Into<Class>>(mut self, classes: T) -> Self {
        for class in classes.into().as_ref().split_whitespace() {
            self = self.class(class.to_string());
        }

        self
    }

    pub fn attr<T: Into<AttributeName>, P: Into<AttributeValue>>(
        mut self,
        name: T,
        value: P,
    ) -> Self {
        match self {
            Builder::Container(ref mut container) => {
                container.attributes.insert(name.into(), value.into());
            }
            Builder::Item(ref mut item) => {
                item.attributes.insert(name.into(), value.into());
            }
            _ => panic!("Only Container and Item can have attributes."),
        }

        self
    }

    pub fn on_click<H>(mut self, handler: H) -> Self
    where
        H: 'static + ClickHandler<A>,
    {
        match self {
            Builder::Container(ref mut container) => container.handlers.click(handler),
            _ => panic!("Only Container and Item can have handlers."),
        }

        self
    }

    pub fn child(mut self, node: Builder<'cpt, S, M, A>) -> Self {
        match self {
            Builder::Container(ref mut container) => {
                container.children.push(node.done());
            }
            _ => panic!("Only Container can have children."),
        };
        self
    }

    pub fn done(self) -> DynamicNode<'cpt, S, M, A> {
        match self {
            Builder::Component(builder) => {
                DynamicNode::component(builder.component, builder.message)
            }
            Builder::Container(builder) => DynamicNode::container(
                builder.name,
                builder.key,
                builder.classes,
                builder.attributes,
                builder.handlers,
                builder.children,
            ),
            Builder::Item(builder) => DynamicNode::item(
                builder.name,
                builder.key,
                builder.classes,
                builder.attributes,
                builder.handlers,
            ),
            Builder::Text(builder) => DynamicNode::text(builder.content, builder.handlers),
        }
    }
}
