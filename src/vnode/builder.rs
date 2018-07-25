use std::sync::Arc;

use super::dynamic_node::DynamicChildren;
use super::DynamicNode;
use super::{
    AttrName, AttrValue, Attributes, Class, Classes, IdValue, KeyValue, Name, TextContent,
};
use super::{Item, Text};

use event::{ClickEvent, Handlers};
use layout::ContainerLayout;
use template::Template;

pub struct BuilderContainer<S, M, A> {
    item: Item<A>,
    children: DynamicChildren<S, M, A>,
    layout: ContainerLayout,
}

pub struct BuilderTemplate<S, M, A> {
    message: Option<M>,
    template: Arc<Template<S, M, A>>,
}

pub enum Builder<S, M, A> {
    Text(Text<A>),
    Item(Item<A>),
    Container(BuilderContainer<S, M, A>),
    Template(BuilderTemplate<S, M, A>),
}

impl<S, M, A> Builder<S, M, A> {
    //
    // # Constructors
    //

    pub fn text<T: Into<TextContent>>(content: T) -> Self {
        Builder::Text(Text {
            content: content.into(),
            handlers: Handlers::new(),
        })
    }

    pub fn item<T: Into<Name>>(name: T) -> Self {
        Builder::Item(Item {
            id: None,
            name: name.into(),
            key: None,
            classes: Classes::new(),
            attributes: Attributes::new(),
            handlers: Handlers::new(),
            layout: Default::default(),
        })
    }

    pub fn container<T: Into<Name>>(name: T) -> Self {
        let item = Item {
            id: None,
            name: name.into(),
            key: None,
            classes: Classes::new(),
            attributes: Attributes::new(),
            handlers: Handlers::new(),
            layout: Default::default(),
        };

        Builder::Container(BuilderContainer {
            item,
            children: DynamicChildren::new(),
            layout: Default::default(),
        })
    }

    //
    // # Properties
    //

    pub fn id<T: Into<IdValue>>(mut self, id: T) -> Self {
        match self {
            Builder::Container(BuilderContainer { ref mut item, .. }) => item.id = Some(id.into()),
            Builder::Item(ref mut item) => item.id = Some(id.into()),
            Builder::Text(_) => panic!("Text builder nodes do not have item properties"),
            Builder::Template(_) => panic!("Template builder nodes do not have item properties"),
        };

        self
    }

    pub fn key<T: Into<KeyValue>>(mut self, key: T) -> Self {
        match self {
            Builder::Container(BuilderContainer { ref mut item, .. }) => {
                item.key = Some(key.into())
            }
            Builder::Item(ref mut item) => item.key = Some(key.into()),
            Builder::Text(_) => panic!("Text builder nodes do not have item properties"),
            Builder::Template(_) => panic!("Template builder nodes do not have item properties"),
        };

        self
    }

    pub fn class<T: Into<Class>>(mut self, class: T) -> Self {
        match self {
            Builder::Container(BuilderContainer { ref mut item, .. }) => {
                item.classes.insert(class.into())
            }
            Builder::Item(ref mut item) => item.classes.insert(class.into()),
            Builder::Text(_) => panic!("Text builder nodes do not have item properties"),
            Builder::Template(_) => panic!("Template builder nodes do not have item properties"),
        };

        self
    }

    pub fn classes<T: Into<Class>>(mut self, classes: T) -> Self {
        for class in classes.into().as_ref().split_whitespace() {
            self = self.class(class.to_string());
        }

        self
    }

    pub fn attr<T: Into<AttrName>, P: Into<AttrValue>>(mut self, name: T, value: P) -> Self {
        match self {
            Builder::Container(BuilderContainer { ref mut item, .. }) => {
                item.attributes.insert(name.into(), value.into());
            }
            Builder::Item(ref mut item) => {
                item.attributes.insert(name.into(), value.into());
            }
            Builder::Text(_) => panic!("Text builder nodes do not have item properties"),
            Builder::Template(_) => panic!("Template builder nodes do not have item properties"),
        };

        self
    }

    pub fn message(mut self, message: M) -> Self {
        match self {
            Builder::Template(ref mut template) => template.message = Some(message),
            _ => panic!("Only Template can have message."),
        }

        self
    }

    pub fn on_click<H>(mut self, handler: H) -> Self
    where
        H: Fn(ClickEvent) -> A + 'static,
    {
        match self {
            Builder::Container(ref mut container) => container.item.handlers.click(handler),
            Builder::Item(ref mut item) => item.handlers.click(handler),
            Builder::Text(ref mut text) => text.handlers.click(handler),
            _ => panic!("Template nodes do not have handlers"),
        }

        self
    }

    pub fn child<T: Into<Builder<S, M, A>>>(mut self, child: T) -> Self {
        match self {
            Builder::Container(ref mut container) => {
                container.children.push(child.into().done());
            }
            _ => panic!("Only Container can have children."),
        }

        self
    }

    //
    // # Converters
    //

    pub fn done(self) -> DynamicNode<S, M, A> {
        match self {
            Builder::Text(text) => DynamicNode::new_text(text),
            Builder::Item(item) => DynamicNode::new_item(item),
            Builder::Container(BuilderContainer {
                item,
                children,
                layout,
            }) => DynamicNode::new_container(item, children, layout),
            Builder::Template(BuilderTemplate { template, message }) => {
                DynamicNode::new_template(template, message)
            }
        }
    }
}

impl<S, M, A> From<Arc<Template<S, M, A>>> for Builder<S, M, A> {
    fn from(template: Arc<Template<S, M, A>>) -> Self {
        Builder::Template(BuilderTemplate {
            template,
            message: None,
        })
    }
}
