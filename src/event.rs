use std::sync::Arc;

pub struct ClickEvent {}

#[derive(Clone)]
pub struct Handlers<A> {
    pub click: Option<Arc<Fn(ClickEvent) -> A>>,
}

impl<A> Default for Handlers<A> {
    fn default() -> Handlers<A> {
        Handlers::new()
    }
}

impl<A> Handlers<A> {
    pub fn new() -> Self {
        Handlers { click: None }
    }

    pub fn click<H>(&mut self, handler: H)
    where
        H: Fn(ClickEvent) -> A + 'static,
    {
        self.click = Some(Arc::new(handler));
    }
}

// pub trait ClickHandler<A>: Fn(ClickEvent) -> A {}
