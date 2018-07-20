use std::sync::Arc;

use super::{Item, Text};
use layout::ContainerLayout;
use template::Template;

use super::static_node::StaticNode;

pub type DynamicText<A> = Arc<Text<A>>;

pub type DynamicItem<A> = Arc<Item<A>>;

pub type DynamicChildren<S, M, A> = Vec<DynamicNode<S, M, A>>;
pub struct DynamicContainer<S, M, A> {
    item: DynamicItem<A>,
    children: DynamicChildren<S, M, A>,
    layout: Arc<ContainerLayout>,
}

pub struct DynamicTemplate<S, M, A> {
    message: Option<M>,
    template: Arc<Template<S, M, A>>,
}

pub enum DynamicNode<S, M, A> {
    Text(DynamicText<A>),
    Item(DynamicItem<A>),
    Container(DynamicContainer<S, M, A>),
    Template(DynamicTemplate<S, M, A>),
}

impl<S, M, A> DynamicNode<S, M, A> {
    pub fn new_text(text: Text<A>) -> Self {
        DynamicNode::Text(Arc::new(text))
    }

    pub fn new_item(item: Item<A>) -> Self {
        DynamicNode::Item(Arc::new(item))
    }

    pub fn new_container(
        item: Item<A>,
        children: DynamicChildren<S, M, A>,
        layout: ContainerLayout,
    ) -> Self {
        DynamicNode::Container(DynamicContainer {
            item: Arc::new(item),
            children,
            layout: Arc::new(layout),
        })
    }

    pub fn new_template(template: Arc<Template<S, M, A>>, message: Option<M>) -> Self {
        DynamicNode::Template(DynamicTemplate { template, message })
    }

    pub fn render(&self, store: &S) -> StaticNode<A> {
        match self {
            DynamicNode::Text(node) => StaticNode::new_text(node.clone()),
            DynamicNode::Item(node) => StaticNode::new_item(node.clone()),
            DynamicNode::Container(DynamicContainer {
                item,
                children,
                layout,
            }) => StaticNode::new_container(
                item.clone(),
                children.iter().map(|child| child.render(store)).collect(),
                layout.clone(),
            ),
            DynamicNode::Template(DynamicTemplate { template, message }) => {
                template.render(store, message).render(store)
            }
        }
    }
}

impl<S, M, A> From<Arc<Template<S, M, A>>> for DynamicNode<S, M, A> {
    fn from(template: Arc<Template<S, M, A>>) -> Self {
        DynamicNode::Template(DynamicTemplate {
            template,
            message: None,
        })
    }
}
