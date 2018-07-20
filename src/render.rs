use std::sync::Arc;

use vnode::{Item, StaticNode, Text};

pub type Position = (usize, usize);
pub type Size = (usize, usize);

pub struct RenderText<A> {
    text: Arc<Text<A>>,
    position: Position,
    size: Size,
}

pub struct RenderItem<A> {
    item: Arc<Item<A>>,
    position: Position,
    size: Size,
}

pub enum RenderCommand<A> {
    Text(RenderText<A>),
    Item(RenderItem<A>),
}

impl<A> RenderCommand<A> {
    fn text(
        text: Arc<Text<A>>,
        parent_position: Option<Position>,
        parent_size: Option<Size>,
    ) -> Self {
        let position = parent_position.unwrap_or((0, 0));
        let size = parent_size.unwrap_or((0, 0));
        RenderCommand::Text(RenderText {
            text,
            position,
            size,
        })
    }
}

pub type RenderList<A> = Vec<RenderCommand<A>>;

pub fn render_list<A>(
    node: StaticNode<A>,
    parent_position: Option<Position>,
    parent_size: Option<Size>,
) -> RenderList<A> {
    let mut list = RenderList::new();
    match node {
        StaticNode::Text(text) => {
            list.push(RenderCommand::text(text, parent_position, parent_size))
        }
        _ => panic!("Not implemented"),
    };

    list
}
