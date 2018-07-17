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

impl<A> Item<A> {
    fn key(&self) -> Option<&str> {
        match self.key {
            Some(ref key) => Some(key),
            None => None,
        }
    }

    fn has_key(&self) -> bool {
        match self.key {
            Some(ref key) => true,
            None => false,
        }
    }

    fn classes(&self) -> &Classes {
        &self.classes
    }

    fn has_class<T: Into<Class>>(&self, class: T) -> bool {
        self.classes.contains(&class.into())
    }

    fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn has_attr<T: Into<AttrName>>(&self, name: T) -> bool {
        self.attributes.contains_key(&name.into())
    }

    fn attr<T: Into<AttrName>>(&self, name: T) -> Option<&str> {
        match self.attributes.get(&name.into()) {
            Some(value) => Some(&value),
            None => None,
        }
    }

    fn handlers(&self) -> &Handlers<A> {
        &self.handlers
    }
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

impl<S, M, A> From<SharedTemplate<S, M, A>> for DynamicNode<S, M, A> {
    fn from(template: SharedTemplate<S, M, A>) -> Self {
        DynamicNode::Template(template, None)
    }
}

type StaticChildren<A> = Vec<StaticNode<A>>;

pub enum StaticNode<A> {
    Text(SharedText<A>),
    Item(SharedItem<A>),
    Container(SharedItem<A>, StaticChildren<A>),
}

impl<A> StaticNode<A> {
    fn name(&self) -> &str {
        match self {
            StaticNode::Item(node) => &node.name,
            StaticNode::Container(node, _) => &node.name,
            _ => panic!("Only Container and Item nodes have name."),
        }
    }

    fn key(&self) -> Option<&str> {
        match self {
            StaticNode::Item(node) => node.key(),
            StaticNode::Container(node, _) => node.key(),
            _ => panic!("Only Container and Item nodes have key."),
        }
    }

    fn has_key(&self) -> bool {
        match self {
            StaticNode::Item(node) => node.has_key(),
            StaticNode::Container(node, _) => node.has_key(),
            _ => false,
        }
    }

    fn classes(&self) -> &Classes {
        match self {
            StaticNode::Item(node) => node.classes(),
            StaticNode::Container(node, _) => node.classes(),
            _ => panic!("Only Container and Item nodes have classes."),
        }
    }

    fn has_class<T: Into<Class>>(&self, class: T) -> bool {
        match self {
            StaticNode::Item(node) => node.has_class(class),
            StaticNode::Container(node, _) => node.has_class(class),
            _ => panic!("Only Container and Item nodes have classes."),
        }
    }

    fn attributes(&self) -> &Attributes {
        match self {
            StaticNode::Item(node) => node.attributes(),
            StaticNode::Container(node, _) => node.attributes(),
            _ => panic!("Only Container and Item nodes have attributes."),
        }
    }

    fn has_attr<T: Into<AttrName>>(&self, name: T) -> bool {
        match self {
            StaticNode::Item(node) => node.has_attr(name),
            StaticNode::Container(node, _) => node.has_attr(name),
            _ => panic!("Only Container and Item nodes have attributes."),
        }
    }

    fn attr<T: Into<AttrName>>(&self, name: T) -> Option<&str> {
        match self {
            StaticNode::Item(node) => node.attr(name),
            StaticNode::Container(node, _) => node.attr(name),
            _ => panic!("Only Container and Item nodes have attributes."),
        }
    }

    fn handlers(&self) -> &Handlers<A> {
        match self {
            StaticNode::Text(node) => &node.handlers,
            StaticNode::Item(node) => node.handlers(),
            StaticNode::Container(node, _) => node.handlers(),
            _ => panic!("Only Container and Item nodes have attributes."),
        }
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
