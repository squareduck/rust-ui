use super::*;
use event::Handlers;
use std::fmt;
use std::sync::Arc;

pub type ChildList<A> = Vec<CachedNode<A>>;

pub struct Container<A> {
    name: Arc<Tag>,
    key: Arc<Key>,
    classes: Arc<ClassList>,
    attributes: Arc<AttributeMap>,
    handlers: Arc<Handlers<A>>,
    children: ChildList<A>,
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

pub enum CachedNode<A> {
    Container(Container<A>),
    Item(Item<A>),
    Text(Text<A>),
}

impl<A> CachedNode<A> {
    pub fn container(
        name: Arc<Tag>,
        key: Arc<Key>,
        classes: Arc<ClassList>,
        attributes: Arc<AttributeMap>,
        handlers: Arc<Handlers<A>>,
        children: ChildList<A>,
    ) -> CachedNode<A> {
        CachedNode::Container(Container {
            name: name,
            key: key,
            classes: classes,
            attributes: attributes,
            handlers: handlers,
            children: children,
        })
    }

    pub fn item(
        name: Arc<Tag>,
        key: Arc<Key>,
        classes: Arc<ClassList>,
        attributes: Arc<AttributeMap>,
        handlers: Arc<Handlers<A>>,
    ) -> CachedNode<A> {
        CachedNode::Item(Item {
            name: name,
            key: key,
            classes: classes,
            attributes: attributes,
            handlers: handlers,
        })
    }

    pub fn text(content: Arc<TextContent>, handlers: Arc<Handlers<A>>) -> CachedNode<A> {
        CachedNode::Text(Text {
            content: content,
            handlers: handlers,
        })
    }
}

impl<A> fmt::Display for CachedNode<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_with_indent(f, None, self)
    }
}

impl<A> fmt::Debug for CachedNode<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_with_indent(f, Some(0), self)
    }
}

fn format_opening_tag(
    f: &mut fmt::Formatter,
    name: &Tag,
    key: &Key,
    classes: &ClassList,
    attributes: &AttributeMap,
) -> fmt::Result {
    write!(f, "<{}", name)?;

    if let Some(key) = key {
        write!(f, " key=\"{}\"", key)?;
    };

    if classes.len() > 0 {
        let mut class_list: Vec<&str> = classes.iter().map(|s| s.as_ref()).collect();
        class_list.sort();
        write!(f, " class=\"{}\"", class_list.join(" "))?;
    }

    if attributes.len() > 0 {
        let mut attr_list: Vec<(&str, &str)> = attributes
            .iter()
            .map(|(n, v)| (n.as_ref(), v.as_ref()))
            .collect();

        attr_list.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (name, value) in attr_list {
            write!(f, " {}", name)?;
            if value.len() > 0 {
                write!(f, "=\"{}\"", value)?;
            }
        }
    }

    write!(f, ">")
}

fn format_closing_tag(f: &mut fmt::Formatter, name: &Tag) -> fmt::Result {
    write!(f, "</{}>", name)
}

fn add_indent(f: &mut fmt::Formatter, indent: Option<usize>) -> fmt::Result {
    match indent {
        Some(level) => {
            for _ in 0..level {
                write!(f, "    ")?;
            }
            Ok(())
        }
        None => Ok(()),
    }
}

pub fn format_with_indent<A>(
    f: &mut fmt::Formatter,
    indent: Option<usize>,
    node: &CachedNode<A>,
) -> fmt::Result {
    let should_indent = match indent {
        Some(_) => true,
        None => false,
    };

    match node {
        CachedNode::Container(node) => {
            add_indent(f, indent)?;
            format_opening_tag(f, &node.name, &node.key, &node.classes, &node.attributes)?;

            if node.children.len() > 0 {
                for child in node.children.iter() {
                    if should_indent {
                        write!(f, "\n")?;
                    }
                    format_with_indent(
                        f,
                        if let Some(indent) = indent {
                            Some(indent + 1)
                        } else {
                            None
                        },
                        &child,
                    )?;
                }

                if should_indent {
                    write!(f, "\n")?;
                    add_indent(f, indent)?;
                }
            }

            format_closing_tag(f, &node.name)
        }
        CachedNode::Item(node) => {
            add_indent(f, indent)?;
            format_opening_tag(f, &node.name, &node.key, &node.classes, &node.attributes)
        }
        CachedNode::Text(node) => {
            add_indent(f, indent)?;
            write!(f, "{}", &node.content)
        }
    }
}
