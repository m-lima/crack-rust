// TODO: Remove this once Path is used instead of Ident in qmetaobject_impl
use qmetaobject::QObject;
use syntaxhighlighter::QSyntaxHighlighter;

use crate::hash;

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
struct HashExtractor {
    base: qmetaobject::qt_base_class!(trait QSyntaxHighlighter),
    color: qmetaobject::qt_property!(qmetaobject::QColor),
    hashes: qmetaobject::qt_method!(fn(&self, text: String) -> qmetaobject::QVariantList),
}

impl QSyntaxHighlighter for HashExtractor {
    fn highlight_block(&mut self, text: String) {
        let regex = <hash::sha256::Hash as hash::Hash>::regex();
        regex.find_iter(&text).for_each(|m| {
            let start = m.start();
            let count = m.end() - start;
            self.format_text(start, count, self.color)
        });
    }
}

impl HashExtractor {
    #[allow(clippy::unused_self, clippy::needless_pass_by_value)]
    fn hashes(&self, text: String) -> qmetaobject::QVariantList {
        let regex = <hash::sha256::Hash as hash::Hash>::regex();
        let set = regex
            .find_iter(&text)
            .map(|m| String::from(m.as_str()))
            .collect::<std::collections::HashSet<_>>();
        set.into_iter().map(qmetaobject::QString::from).collect()
    }
}

pub fn run() {
    let cracker = std::ffi::CString::new("Cracker").unwrap();
    let hash_extractor = std::ffi::CString::new("HashExtractor").unwrap();

    qml();
    img();

    let templates = qmetaobject::QObjectBox::new(
        include!("../../hidden/template.in")
            .iter()
            .collect::<qmetaobject::SimpleListModel<_>>(),
    );

    let mut engine = qmetaobject::QmlEngine::new();
    qmetaobject::qml_register_singleton_type::<Cracker>(&cracker, 1, 0, &cracker);
    qmetaobject::qml_register_type::<HashExtractor>(&hash_extractor, 1, 0, &hash_extractor);
    engine.set_object_property("_templates".into(), templates.pinned());
    engine.load_file("qrc:/Main.qml".into());
    engine.exec();
}
