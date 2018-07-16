use node::CachedNode;
use node::DynamicNode;
use std::sync::Arc;

pub struct Component<'cpt, S, M, A>
where
    S: 'cpt,
    M: 'cpt,
    A: 'cpt,
{
    renderer: Box<Fn(&S, &Option<M>) -> DynamicNode<'cpt, S, M, A>>,
    node_cache: Option<CachedNode<A>>,
}

impl<'cpt, S, M, A> Component<'cpt, S, M, A> {
    pub fn new<F>(renderer: F) -> Arc<Self>
    where
        F: 'static + Fn(&S, &Option<M>) -> DynamicNode<'cpt, S, M, A>,
    {
        Arc::new(Component {
            renderer: Box::new(renderer),
            node_cache: None,
        })
    }

    pub fn render(&self, store: &S, message: &Option<M>) -> CachedNode<A> {
        (self.renderer)(store, message).render(store)
    }
}
