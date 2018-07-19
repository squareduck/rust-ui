use super::StaticNode;
use super::{Attributes, Classes, Id, Key, Name};
use std::fmt;

fn format_opening_tag(
    f: &mut fmt::Formatter,
    name: &Name,
    id: &Id,
    key: &Key,
    classes: &Classes,
    attributes: &Attributes,
) -> fmt::Result {
    write!(f, "<{}", name)?;

    if let Some(id) = id {
        write!(f, " id=\"{}\"", id)?;
    };

    if let Some(key) = key {
        write!(f, " key=\"{}\"", key)?;
    };

    if !classes.is_empty() {
        let mut class_list: Vec<&str> = classes.iter().map(|s| s.as_ref()).collect();
        class_list.sort();
        write!(f, " class=\"{}\"", class_list.join(" "))?;
    }

    if !attributes.is_empty() {
        let mut attr_list: Vec<(&str, &str)> = attributes
            .iter()
            .map(|(n, v)| (n.as_ref(), v.as_ref()))
            .collect();

        attr_list.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (name, value) in attr_list {
            write!(f, " {}", name)?;
            if !value.is_empty() {
                write!(f, "=\"{}\"", value)?;
            }
        }
    }

    write!(f, ">")
}

fn format_closing_tag(f: &mut fmt::Formatter, name: &Name) -> fmt::Result {
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
    node: &StaticNode<A>,
) -> fmt::Result {
    let should_indent = match indent {
        Some(_) => true,
        None => false,
    };

    match node {
        StaticNode::Container(item, children) => {
            add_indent(f, indent)?;
            format_opening_tag(
                f,
                &item.name,
                &item.id,
                &item.key,
                &item.classes,
                &item.attributes,
            )?;

            if !children.is_empty() {
                for child in children.iter() {
                    if should_indent {
                        writeln!(f)?;
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
                    writeln!(f)?;
                    add_indent(f, indent)?;
                }
            }

            format_closing_tag(f, &item.name)
        }
        StaticNode::Item(item) => {
            add_indent(f, indent)?;
            format_opening_tag(
                f,
                &item.name,
                &item.id,
                &item.key,
                &item.classes,
                &item.attributes,
            )
        }
        StaticNode::Text(text) => {
            add_indent(f, indent)?;
            write!(f, "{}", &text.content)
        }
    }
}
