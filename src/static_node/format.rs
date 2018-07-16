use super::*;
use std::fmt;

impl<'node, A> fmt::Display for StaticNode<'node, A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_with_indent(f, None, self)
    }
}

impl<'node, A> fmt::Debug for StaticNode<'node, A> {
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
    node: &StaticNode<A>,
) -> fmt::Result {
    let should_indent = match indent {
        Some(_) => true,
        None => false,
    };

    match node {
        StaticNode::Container(node) => {
            add_indent(f, indent)?;
            format_opening_tag(f, node.name, node.key, node.classes, node.attributes)?;

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

            format_closing_tag(f, node.name)
        }
        StaticNode::Item(node) => {
            add_indent(f, indent)?;
            format_opening_tag(f, node.name, node.key, node.classes, node.attributes)
        }
        StaticNode::Text(node) => {
            add_indent(f, indent)?;
            write!(f, "{}", &node.content)
        }
    }
}
