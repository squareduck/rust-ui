use event::Handlers;
use layout::ItemLayout;
use std::collections::{HashMap, HashSet};
use types::CowString;

pub mod builder;
pub mod dynamic_node;
mod format;
pub mod static_node;

pub use self::dynamic_node::DynamicNode;
pub use self::static_node::StaticNode;

pub type TextContent = CowString;
pub type Name = CowString;
pub type KeyValue = CowString;
pub type Key = Option<KeyValue>;
pub type IdValue = CowString;
pub type Id = Option<CowString>;
pub type Class = CowString;
pub type Classes = HashSet<Class>;
pub type AttrName = CowString;
pub type AttrValue = CowString;
pub type Attributes = HashMap<AttrName, AttrValue>;

pub struct Text<A> {
    content: TextContent,
    handlers: Handlers<A>,
}

pub struct Item<A> {
    id: Id,
    name: Name,
    key: Key,
    classes: Classes,
    attributes: Attributes,
    handlers: Handlers<A>,
    layout: ItemLayout,
}

impl<A> Item<A> {
    fn id(&self) -> Option<&str> {
        match self.id.as_ref() {
            Some(id) => Some(id),
            None => None,
        }
    }

    fn has_id(&self) -> bool {
        match self.id {
            Some(_) => true,
            None => false,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn key(&self) -> Option<&str> {
        match self.key.as_ref() {
            Some(key) => Some(key),
            None => None,
        }
    }

    fn has_key(&self) -> bool {
        match self.key {
            Some(_) => true,
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
