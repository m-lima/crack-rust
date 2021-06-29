use cpp::cpp;

use qmetaobject::*;

/// This trait allow to override a Qt QSyntaxHighlighter
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

    // /// Refer to the Qt documentation of QSyntaxHighlighter::document
    // fn document(&self) -> Option<QQuickTextDocument> {
    //     let obj = self.get_cpp_object();
    //     let text_document = cpp!(unsafe [obj as "Rust_QSyntaxHighlighter *"] -> QQuickTextDocument as "const QQuickTextDocument *" {
    //         return obj ? obj->quickDocument() : QQuickTextDocument();
    //     });
    // }

    // /// Refer to the Qt documentation of QSyntaxHighlighter::setDocument
    // fn set_document(&mut self, document: QQuickTextDocument) -> QModelIndex {
    //     let obj = self.get_cpp_object();
    //     cpp!(unsafe [obj as "Rust_QSyntaxHighlighter *", document as "QQuickTextDocument"] -> QQuickTextDocument as "QQuickTextDocument" {
    //         return obj ? obj->document() : QQuickTextDocument();
    //     })
    // }

    fn highlight_block(&mut self, text: String);
}

// cpp_class!(
//     #[derive(Default, Clone, Copy, PartialEq)]
//     pub unsafe struct QQuickTextDocument as "QQuickTextDocument"
// );

cpp! {{
    #include <QtGui/QSyntaxHighlighter>
    #include <QtQuick/QQuickTextDocument>

    class Rust_QSyntaxHighlighterProxy : public QSyntaxHighlighter {
        Q_OBJECT

        Q_PROPERTY(QQuickTextDocument *textDocument READ textDocument WRITE setTextDocument NOTIFY textDocumentChanged)

        public:
        explicit Rust_QSyntaxHighlighterProxy(QObject *parent = nullptr) : QSyntaxHighlighter(parent) {
            m_TextDocument = nullptr;
        }

        signals:
        void textDocumentChanged();

        protected:

        void setTextDocument(QQuickTextDocument *textDocument) {
            if (m_TextDocument == textDocument) {
                return;
            }

            m_TextDocument = textDocument;

            setDocument(m_TextDocument ? m_TextDocument->textDocument() : nullptr);

            emit textDocumentChanged();
        }

        QQuickTextDocument *textDocument() {
            return m_TextDocument;
        }

        QQuickTextDocument *m_TextDocument;
    };
}}

cpp! {{
    #include <qmetaobject_rust.hpp>

    struct Rust_QSyntaxHighlighter : RustObject<Rust_QSyntaxHighlighterProxy> {

        using Rust_QSyntaxHighlighterProxy::setTextDocument;
        using Rust_QSyntaxHighlighterProxy::textDocument;

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
