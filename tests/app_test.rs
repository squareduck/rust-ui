#[macro_use]
extern crate pretty_assertions;
extern crate cinnabar;

use self::helper::{button, panel, text};
use self::helper::{Action, Store};

use cinnabar::{App, Template};

mod helper;

#[test]
fn simple_app_test() {
    let store = Store { points: 0 };

    let counter = Template::new(|store: &Store, _message| {
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
