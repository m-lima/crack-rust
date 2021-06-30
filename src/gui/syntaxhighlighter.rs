use cpp::cpp;

pub trait QSyntaxHighlighter: qmetaobject::QObject {
    fn get_object_description() -> &'static qmetaobject::QObjectDescription
    where
        Self: Sized,
    {
        unsafe {
            &*cpp::cpp!([]-> *const qmetaobject::QObjectDescription as "RustObjectDescription const*" {
                return rustObjectDescription<Rust_QSyntaxHighlighter>();
            })
        }
    }

    fn highlight_block(&mut self, text: String);

    // Allowed because we check it and i32::MAX is always positive
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn format_text(&self, start: usize, count: usize, color: qmetaobject::QColor) {
        let obj = self.get_cpp_object();
        let start = if start > i32::MAX as usize {
            panic!("Index overflow")
        } else {
            start as i32
        };
        let count = if count > i32::MAX as usize {
            panic!("Count overflow")
        } else {
            count as i32
        };
        cpp::cpp!(unsafe [obj as "Rust_QSyntaxHighlighter*", start as "int", count as "int", color as "QColor"] {
            if (obj) obj->setColorFormat(start, count, color);
        });
    }
}

cpp::cpp! {{
    #include <qmetaobject_rust.hpp>
    #include <QSyntaxHighlighterProxy.cpp>
}}

cpp::cpp! {{
    struct Rust_QSyntaxHighlighter : public RustObject<QSyntaxHighlighterProxy> {
        void highlightBlock(const QString &text) override {
            return rust!(Rust_QSyntaxHighlighter_highlightBlock [
                rust_object: qmetaobject::QObjectPinned<'_, dyn QSyntaxHighlighter> as "TraitObject",
                text: qmetaobject::QString as "QString"
            ] {
                rust_object.borrow_mut().highlight_block(text.clone().into())
            });
        }
    };
}}
