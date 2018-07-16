use std::sync::Arc;

pub struct Handlers<A> {
    pub click: Option<Arc<ClickHandler<A, Output = A>>>,
}

impl<A> Handlers<A> {
    pub fn new() -> Self {
        Handlers { click: None }
    }

    pub fn click<H>(&mut self, handler: H)
    where
        H: 'static + ClickHandler<A>,
    {
        self.click = Some(Arc::new(handler));
    }
}

pub struct ClickEvent {}
pub trait ClickHandler<A>: Fn(ClickEvent) -> A {}
