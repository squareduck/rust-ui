#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate cinnabar;

mod helper;

use self::helper::{Action, Message, Store};

use cinnabar::Template;

elements_for!(Store, Message, Action);

#[test]
fn simple_node() {
    let dn = panel()
        .child(text("Please press OK."))
        .child(
            button()
                .key("b1")
                .attr("hidden", "")
                .child(text("No thanks.")),
        )
        .child(button().key("b2").class("primary").child(text("OK!")))
        .done();
    let sn = dn.render(&Store { points: 0 });
    assert_eq!(
        format!("\n{:?}\n", sn),
        r#"
<panel>
    Please press OK.
    <button key="b1" hidden>
        No thanks.
    </button>
    <button key="b2" class="primary">
        OK!
    </button>
</panel>
"#
    );
}

#[test]
fn dynamic_node() {
    let mut store = Store { points: 0 };

    let counter = Template::new(|store: &Store, message: &Option<Message>| {
        let p = format!("{}", store.points);
        text(p).done()
    });

    let dn = panel().child(counter.clone()).done();

    let sn = dn.render(&store);

    assert_eq!(
        format!("\n{:?}\n", sn),
        r#"
<panel>
    0
</panel>
"#
    );

    store = Store { points: 1 };

    let sn = dn.render(&store);

    assert_eq!(
        format!("\n{:?}\n", sn),
        r#"
<panel>
    1
</panel>
"#
    );
}
