use render::{render_list, Position, RenderList, Size};
use vnode::{DynamicNode, StaticNode};

pub struct App<S, M, A> {
    store: S,
    vnode: DynamicNode<S, M, A>,
    reducer: Box<Fn(S, A) -> S + 'static>,
    last_render: StaticNode<A>,
}

impl<S, M, A> App<S, M, A> {
    pub fn new<T: Into<DynamicNode<S, M, A>>>(
        store: S,
        vnode: T,
        reducer: impl Fn(S, A) -> S + 'static,
    ) -> App<S, M, A> {
        let dynamic_node = vnode.into();

        let last_render = dynamic_node.render(&store);

        App {
            store,
            vnode: dynamic_node,
            reducer: Box::new(reducer),
            last_render,
        }
    }

    pub fn view(&self) -> &StaticNode<A> {
        &self.last_render
    }

    pub fn render_list(&self, position: Position, size: Size) -> RenderList<A> {
        render_list(&self.last_render, position, size)
    }

    pub fn action(mut self, action: A) -> Self {
        self.store = (self.reducer)(self.store, action);
        self.last_render = self.vnode.render(&self.store);
        self
    }
}
