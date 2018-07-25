use vnode::static_node::{StaticItem, StaticNode, StaticText};

pub type Position = (u16, u16);
pub type Size = (u16, u16);

pub enum RenderItem<A> {
    Text(StaticText<A>),
    Item(StaticItem<A>),
}

pub struct RenderText<A> {
    pub position: Position,
    pub size: Size,
    pub node: StaticText<A>,
}

pub enum RenderCommand<A> {
    Text(RenderText<A>),
}

impl<A> RenderCommand<A> {
    fn text(text: StaticText<A>, parent_position: Position, parent_size: Size) -> Self {
        let position = parent_position;
        let size = (text.content().len() as u16, 1 as u16);
        RenderCommand::Text(RenderText {
            node: text,
            position,
            size,
        })
    }
}

pub type RenderList<A> = Vec<RenderCommand<A>>;

pub fn render_list<A>(
    node: &StaticNode<A>,
    parent_position: Position,
    parent_size: Size,
) -> RenderList<A> {
    let mut list = RenderList::new();
    match node {
        StaticNode::Text(text) => list.push(RenderCommand::text(
            text.clone(),
            parent_position,
            (text.content().len() as u16, 1 as u16),
        )),
        _ => panic!("Not implemented"),
    };

    list
}
