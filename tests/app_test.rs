#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate cinnabar;

mod helper;

use self::helper::{Action, Message, Store};

use cinnabar::{App, Template};

elements_for!(Store, Message, Action);

#[test]
fn simple_app_test() {
    let store = Store { points: 0 };

    let counter = Template::new(|store: &Store, _message: &Option<Message>| {
        let points = format!("{} points", store.points);
        panel()
            .child(text(points))
            .child(button().child(text("Increment")))
            .done()
    });

    let mut app = App::new(store, counter.clone(), |store, action| {
        use self::Action::*;
        match action {
            Increment => Store {
                points: store.points + 1,
            },
            None => store,
        }
    });

    app = app.action(Action::None);

    assert_eq!(
        format!("\n{:?}\n", app.view()),
        r#"
<panel>
    0 points
    <button>
        Increment
    </button>
</panel>
"#
    );

    app = app.action(Action::Increment);

    assert_eq!(
        format!("\n{:?}\n", app.view()),
        r#"
<panel>
    1 points
    <button>
        Increment
    </button>
</panel>
"#
    );
}
