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
}

cpp! {{
    #include <qmetaobject_rust.hpp>
    #include <QtQuick/QQuickTextDocument>
    #include <QtGui/QSyntaxHighlighter>
}}

cpp! {{
    struct Rust_QSyntaxHighlighterProxy : public QSyntaxHighlighter {
        explicit Rust_QSyntaxHighlighterProxy(QObject *parent = nullptr) : QSyntaxHighlighter(parent) {}
    };

    class Rust_QSyntaxHighlighter : public RustObject<Rust_QSyntaxHighlighterProxy> {
        Q_PROPERTY(QQuickTextDocument *textDocument READ textDocument WRITE setTextDocument NOTIFY textDocumentChanged)

        public:
        Rust_QSyntaxHighlighter() : RustObject<Rust_QSyntaxHighlighterProxy>() {
            m_TextDocument = nullptr;
        }

        void highlightBlock(const QString &text) override {
            return rust!(Rust_QSyntaxHighlighter_highlightBlock [
                rust_object: QObjectPinned<'_, dyn QSyntaxHighlighter> as "TraitObject",
                text: QString as "QString"
            ] {
                rust_object.borrow_mut().highlight_block(text.clone().into())
            });
        }

        QQuickTextDocument *textDocument() const {
            return m_TextDocument;
        }

        void setTextDocument(QQuickTextDocument *textDocument) {
            if (m_TextDocument == textDocument) {
                return;
            }

            m_TextDocument = textDocument;

            setDocument(m_TextDocument ? m_TextDocument->textDocument() : nullptr);

            emit textDocumentChanged();
        }

        signals:
        void textDocumentChanged();

        private:
        QQuickTextDocument *m_TextDocument;
    };
}}
