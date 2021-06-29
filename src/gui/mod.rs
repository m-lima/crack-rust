// TODO: Remove this once Path is used instead of Ident in qmetaobject_impl
use qmetaobject::QObject;

qmetaobject::qrc!(qml, "res/qml" as "/" {
    "qtquickcontrols2.conf",
    "Main.qml",
    "BigButton.qml",
    "CollapsibleItem.qml",
    "Input.qml",
    "Navigation.qml",
    "Parameters.qml",
    "Radio.qml",
    "TitleButton.qml",
});

qmetaobject::qrc!(img, "res/img" as "/img" {
    "left.svg",
    "trash.svg",
});

#[derive(qmetaobject::SimpleListItem, Clone, Default)]
struct Template {
    pub name: String,
    pub prefix: String,
    pub length: u8,
}

impl Template {
    fn new(name: &str, prefix: &str, length: u8) -> Self {
        Self {
            name: String::from(name),
            prefix: String::from(prefix),
            length,
        }
    }
}

#[derive(qmetaobject::QObject, Default)]
struct Cracker {
    base: qmetaobject::qt_base_class!(trait QObject),
    crack: qmetaobject::qt_method!(
        #[allow(clippy::unused_self, clippy::needless_pass_by_value)]
        fn crack(&self, prefix: String, length: u8) {
            println!("Prefix: {}, Length: {}", prefix, length);
        }
    ),
}

impl qmetaobject::QSingletonInit for Cracker {
    fn init(&mut self) {}
}

pub trait QSyntaxHighlighter: QObject {
    fn get_object_description() -> &'static qmetaobject::QObjectDescription
    where
        Self: Sized,
    {
        unsafe {
            &*cpp::cpp!([]-> *const qmetaobject::QObjectDescription as "RustObjectDescription const*" {
                return rustObjectDescription<Hasher_QSyntaxHighlighter>();
            })
        }
    }

    fn highlight_block(&mut self, text: String);

    fn set_format(&mut self, start: i32, length: i32, color: qmetaobject::QColor) {
        let obj = qmetaobject::QObject::get_cpp_object(self);
        unsafe {
            cpp::cpp!([obj as "Hasher_QSyntaxHighlighter*", start as "int", length as "int", color as "QColor"] {
                if (obj) obj->setFormatPublic(start, length, color);
            })
        }
    }
}

cpp::cpp! {{
#include <QtGui/QSyntaxHighlighter>

struct Abstract_Hasher_QSyntaxHighlighter : QSyntaxHighlighter {
    Abstract_Hasher_QSyntaxHighlighter() : QSyntaxHighlighter((QObject*) nullptr) {}
};
}}

cpp::cpp! {{
#include <qmetaobject_rust.hpp>
#include <QtQuick/QQuickTextDocument>
#include <QtGui/QTextDocument>

struct Hasher_QSyntaxHighlighter : RustObject<Abstract_Hasher_QSyntaxHighlighter> {
    void highlightBlock(const QString &text) {
        rust!(Hasher_QSyntaxHighlighter_highlightBlock [rust_object: qmetaobject::QObjectPinned<'_, dyn QSyntaxHighlighter> as "TraitObject", text: qmetaobject::QString as "QString"] {
            rust_object.borrow_mut().highlight_block(text.clone().into())
        });
    }

    void setFormatPublic(int start, int length, QColor color) {
        setFormat(start, length, color);
    }

    QQuickTextDocument * textDocument() {
        return m_TextDocument;
    }

    void setTextDocument(QQuickTextDocument * textDocument) {
        if (textDocument == m_TextDocument) {
            return;
        }

        m_TextDocument = textDocument;

        QTextDocument * doc = m_TextDocument->textDocument();
        setDocument(doc);
    }

    QQuickTextDocument* m_TextDocument;
};
}}

cpp::cpp_class!(unsafe struct QQuickTextDocument as "QQuickTextDocument");

#[derive(qmetaobject::QObject, Default)]
struct HashSyntaxHighlighter {
    base: qmetaobject::qt_base_class!(trait QSyntaxHighlighter),
    document: qmetaobject::qt_property!(QQuickTextDocument textDocument),
}

impl QSyntaxHighlighter for HashSyntaxHighlighter {
    #[allow(clippy::cast_possible_truncation)]
    fn highlight_block(&mut self, text: String) {
        self.set_format(
            0,
            text.len() as i32,
            qmetaobject::QColor::from_name("green"),
        )
    }
}

pub fn run() {
    let cracker = std::ffi::CString::new("Cracker").unwrap();
    let hash_highlighter = std::ffi::CString::new("HashHighlighter").unwrap();

    qml();
    img();

    let templates = qmetaobject::QObjectBox::new(
        include!("../../hidden/template.in")
            .iter()
            .collect::<qmetaobject::SimpleListModel<_>>(),
    );

    let mut engine = qmetaobject::QmlEngine::new();
    qmetaobject::qml_register_singleton_type::<Cracker>(&cracker, 1, 0, &cracker);
    qmetaobject::qml_register_type::<HashSyntaxHighlighter>(
        &hash_highlighter,
        1,
        0,
        &hash_highlighter,
    );
    engine.set_object_property("_templates".into(), templates.pinned());
    engine.load_file("qrc:/Main.qml".into());
    engine.exec();
}
