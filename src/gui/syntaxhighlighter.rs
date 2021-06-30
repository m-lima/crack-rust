use cpp::cpp;

use qmetaobject::*;

/// This trait allows to override a Qt QSyntaxHighlighter
pub trait QSyntaxHighlighter: QObject {
    /// Required for the implementation detail of the QObject custom derive
    fn get_object_description() -> &'static QObjectDescription
    where
        Self: Sized,
    {
        unsafe {
            &*cpp!([]-> *const QObjectDescription as "RustObjectDescription const*" {
                return rustObjectDescription<Rust_QSyntaxHighlighter>();
            })
        }
    }

    fn highlight_block(&mut self, text: String);

    fn format_text(&self, start: i32, length: i32, color: QColor) {
        let obj = self.get_cpp_object();
        cpp!([obj as "Rust_QSyntaxHighlighter*", start as "int", length as "int", color as "QColor"] {
            if (obj) obj->setFormat(start, length, color);
        });
    }
}

cpp! {{
    #include <qmetaobject_rust.hpp>
    #include <QSyntaxHighligherProxy.cpp>
}}

cpp! {{
    struct Rust_QSyntaxHighlighter : public RustObject<QSyntaxHighlighterProxy> {
        void highlightBlock(const QString &text) override {
            return rust!(Rust_QSyntaxHighlighter_highlightBlock [
                rust_object: QObjectPinned<'_, dyn QSyntaxHighlighter> as "TraitObject",
                text: QString as "QString"
            ] {
                rust_object.borrow_mut().highlight_block(text.clone().into())
            });
        }
    };
}}
