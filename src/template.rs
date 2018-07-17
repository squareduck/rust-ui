use std::sync::Arc;
use vnode::DynamicNode;

pub type SharedTemplate<S, M, A> = Arc<Template<S, M, A>>;

pub struct Template<S, M, A> {
    renderer: Box<Fn(&S, &Option<M>) -> DynamicNode<S, M, A>>,
}

impl<S, M, A> Template<S, M, A> {
    pub fn render(&self, store: &S, message: &Option<M>) -> DynamicNode<S, M, A> {
        (self.renderer)(store, message)
    }
}

pub fn template<'tpl, S, M, A>(
    renderer: impl 'static + Fn(&S, &Option<M>) -> DynamicNode<S, M, A>,
) -> SharedTemplate<S, M, A> {
    Arc::new(Template {
        renderer: Box::new(renderer),
    })
}
