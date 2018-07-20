use super::static_node::StaticItem;
use super::StaticNode;
use std::fmt;

fn format_opening_tag<A>(f: &mut fmt::Formatter, item: &StaticItem<A>) -> fmt::Result {
    write!(f, "<{}", item.name())?;

    if let Some(id) = item.id() {
        write!(f, " id=\"{}\"", id)?;
    };

    if let Some(key) = item.key() {
        write!(f, " key=\"{}\"", key)?;
    };

    if !item.classes().is_empty() {
        let mut class_list: Vec<&str> = item.classes().iter().map(|s| s.as_ref()).collect();
        class_list.sort();
        write!(f, " class=\"{}\"", class_list.join(" "))?;
    }

    if !item.attributes().is_empty() {
        let mut attr_list: Vec<(&str, &str)> = item.attributes()
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

fn format_closing_tag<A>(f: &mut fmt::Formatter, item: &StaticItem<A>) -> fmt::Result {
    write!(f, "</{}>", item.name())
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
        StaticNode::Container(container) => {
            add_indent(f, indent)?;
            format_opening_tag(f, container.item())?;

            if !container.children().is_empty() {
                for child in container.children().iter() {
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

            format_closing_tag(f, container.item())
        }
        StaticNode::Item(item) => {
            add_indent(f, indent)?;
            format_opening_tag(f, item)
        }
        StaticNode::Text(text) => {
            add_indent(f, indent)?;
            write!(f, "{}", &text.content)
        }
    }
}
