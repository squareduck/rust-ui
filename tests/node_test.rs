#[macro_use]
extern crate pretty_assertions;

extern crate cinnabar;

use cinnabar::node::TextContent;
use cinnabar::Builder;
use cinnabar::Component;

struct Store {
    points: usize,
}

enum Message {}
enum Action {}

fn panel<'node>() -> Builder<'node, Store, Message, Action> {
    Builder::container("panel")
}

fn button<'node>() -> Builder<'node, Store, Message, Action> {
    Builder::container("button")
}

fn text<'node, T: Into<TextContent>>(content: T) -> Builder<'node, Store, Message, Action> {
    Builder::text(content.into())
}

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

    let counter_cpt = Component::new(|store: &Store, message| {
        let p = format!("{}", store.points);
        text(p).done()
    });

    let dn = panel().child(Builder::component(counter_cpt)).done();

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
