// TODO: Rethink event system
// Should host trigger events?
// Sholud they be wrapped into closures/functions and stored in VNode?

pub struct Handlers<A> {
    pub click: Option<Box<ClickHandler<A, Output = A>>>,
}

impl<A> Handlers<A> {
    pub fn new() -> Self {
        Handlers { click: None }
    }

    pub fn click<H>(&mut self, handler: H)
    where
        H: ClickHandler<A>,
    {
        self.click = Some(Box::new(handler));
    }
}

pub struct ClickEvent {}
pub trait ClickHandler<A>: 'static + Fn(ClickEvent) -> A {}
