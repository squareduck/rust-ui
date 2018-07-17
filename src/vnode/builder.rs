use super::DynamicNode;
use super::{AttrName, AttrValue, Attributes, Class, Classes, KeyValue, Name, TextContent};
use super::{Children, Item, Text};
use event::{ClickHandler, Handlers};
use std::sync::Arc;
use template::SharedTemplate;

pub enum Builder<S, M, A> {
    Text(Text<A>),
    Item(Item<A>),
    Container(Item<A>, Children<S, M, A>),
    Template(SharedTemplate<S, M, A>, Option<M>),
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
            name: name.into(),
            key: None,
            classes: Classes::new(),
            attributes: Attributes::new(),
            handlers: Handlers::new(),
        })
    }

    pub fn container<T: Into<Name>>(name: T) -> Self {
        let item = Item {
            name: name.into(),
            key: None,
            classes: Classes::new(),
            attributes: Attributes::new(),
            handlers: Handlers::new(),
        };

        Builder::Container(item, Children::new())
    }

    //
    // # Properties
    //

    pub fn key<T: Into<KeyValue>>(mut self, key: T) -> Self {
        match self {
            Builder::Container(ref mut item, _) => {
                item.key = Some(key.into());
            }
            Builder::Item(ref mut item) => item.key = Some(key.into()),
            _ => panic!("Only Container and Item can have a key."),
        }
        self
    }

    pub fn class<T: Into<Class>>(mut self, class: T) -> Self {
        match self {
            Builder::Container(ref mut item, _) => {
                item.classes.insert(class.into());
            }
            Builder::Item(ref mut item) => {
                item.classes.insert(class.into());
            }
            _ => panic!("Only Container and Item can have a class."),
        }

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
            Builder::Container(ref mut item, _) => {
                item.attributes.insert(name.into(), value.into());
            }
            Builder::Item(ref mut item) => {
                item.attributes.insert(name.into(), value.into());
            }
            _ => panic!("Only Container and Item can have an attribute."),
        }

        self
    }

    pub fn message(mut self, message: M) -> Self {
        let new_builder = match self {
            Builder::Template(tpl, _) => Builder::Template(tpl, Some(message)),
            _ => panic!("Only Template can have message."),
        };

        self = new_builder;

        self
    }

    pub fn on_click<H>(mut self, handler: H) -> Self
    where
        H: 'static + ClickHandler<A>,
    {
        match self {
            Builder::Container(ref mut item, _) => item.handlers.click(handler),
            Builder::Item(ref mut item) => item.handlers.click(handler),
            _ => panic!("Only Container and Item can have handlers."),
        }

        self
    }

    pub fn child<T: Into<Builder<S, M, A>>>(mut self, child: T) -> Self {
        match self {
            Builder::Container(_, ref mut children) => {
                children.push(child.into().done());
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
            Builder::Text(text) => DynamicNode::Text(Arc::new(Text {
                content: text.content,
                handlers: text.handlers,
            })),
            Builder::Item(item) => DynamicNode::Item(Arc::new(Item {
                name: item.name,
                key: item.key,
                classes: item.classes,
                attributes: item.attributes,
                handlers: item.handlers,
            })),
            Builder::Container(item, children) => DynamicNode::Container(
                Arc::new(Item {
                    name: item.name,
                    key: item.key,
                    classes: item.classes,
                    attributes: item.attributes,
                    handlers: item.handlers,
                }),
                children,
            ),
            Builder::Template(template, message) => DynamicNode::Template(template, message),
        }
    }
}

impl<S, M, A> From<SharedTemplate<S, M, A>> for Builder<S, M, A> {
    fn from(template: SharedTemplate<S, M, A>) -> Self {
        Builder::Template(template, None)
    }
}
