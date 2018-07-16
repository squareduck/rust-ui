use std::collections::{HashMap, HashSet};
use types::CowString;

pub use self::cached::CachedNode;
pub use self::dynamic::DynamicNode;

pub mod builder;
pub mod cached;
pub mod dynamic;

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
