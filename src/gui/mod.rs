// TODO: Remove this once Path is used instead of Ident in qmetaobject_impl
use qmetaobject::QObject;
use syntaxhighlighter::QSyntaxHighlighter;

mod syntaxhighlighter;

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

#[allow(non_snake_case)]
#[derive(qmetaobject::QObject, Default)]
struct HashHighlighter {
    base: qmetaobject::qt_base_class!(trait QSyntaxHighlighter),
}

impl QSyntaxHighlighter for HashHighlighter {
    fn highlight_block(&mut self, text: String) {
        let regex = <crate::hash::md5::Hash as crate::hash::Hash>::regex();
        regex.find_iter(text).for_each(|m| {
            let start = m.start() as i32;
            let length = m.end() as i32 - start;
            self.set_format(start, length, qmetaobject::QColor::from_name("green"))
        });
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
    qmetaobject::qml_register_type::<HashHighlighter>(&hash_highlighter, 1, 0, &hash_highlighter);
    engine.set_object_property("_templates".into(), templates.pinned());
    engine.load_file("qrc:/Main.qml".into());
    engine.exec();
}
