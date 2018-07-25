#[macro_export]
macro_rules! elements_for {
    ($s:ident, $m:ident, $a:ident) => {
        use cinnabar::vnode::builder::Builder;
        use cinnabar::vnode::TextContent;

        pub fn panel() -> Builder<$s, $m, $a> {
            Builder::<$s, $m, $a>::container("panel")
        }

        pub fn button() -> Builder<$s, $m, $a> {
            Builder::<$s, $m, $a>::container("button")
        }

        pub fn text<T: Into<TextContent>>(content: T) -> Builder<$s, $m, $a> {
            Builder::<$s, $m, $a>::text(content.into())
        }
    };
}
