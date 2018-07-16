use super::*;

use event::ClickHandler;
use event::Handlers;

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

pub struct ContainerBuilder<'node, S, M, A>
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

pub enum Builder<'node, S, M, A>
where
    A: 'node,
{
    Component(Template<'node, S, M, A>, Option<M>),
    Container(ContainerBuilder<'node, S, M, A>),
    Item(ItemBuilder<A>),
    Text(TextBuilder<A>),
}

impl<'node, S, M, A> Builder<'node, S, M, A> {
    pub fn component(template: Template<'node, S, M, A>) -> Self {
        Builder::Component(template, None)
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
        H: ClickHandler<A>,
    {
        match self {
            Builder::Container(ref mut container) => container.handlers.click(handler),
            _ => panic!("Only Container and Item can have handlers."),
        }

        self
    }

    pub fn child(mut self, node: Builder<'node, S, M, A>) -> Self
    where
        A: 'node,
    {
        match self {
            Builder::Container(ref mut container) => {
                container.children.push(node.done());
            }
            _ => panic!("Only Container can have children."),
        };
        self
    }

    pub fn done(self) -> DynamicNode<'node, S, M, A>
    where
        A: 'node,
    {
        match self {
            Builder::Component(template, message) => DynamicNode::Component(template, message),
            Builder::Container(builder) => DynamicNode::Container(Container {
                name: builder.name,
                key: builder.key,
                classes: builder.classes,
                attributes: builder.attributes,
                handlers: builder.handlers,
                children: builder.children,
            }),
            Builder::Item(builder) => DynamicNode::Item(Item {
                name: builder.name,
                key: builder.key,
                classes: builder.classes,
                attributes: builder.attributes,
                handlers: builder.handlers,
            }),
            Builder::Text(builder) => DynamicNode::Text(Text {
                content: builder.content,
                handlers: builder.handlers,
            }),
        }
    }
}
